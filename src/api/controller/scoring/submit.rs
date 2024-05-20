use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use chrono::Local;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::database::model::{Member, Question, Room, Score, SubmitHistory, Template, TestCase};
use crate::enums::{ProgrammingLanguage, RoomKind};
use crate::util::{self, scoring::ExecutionResult};
use crate::{Error, Result};

use super::SubmitData;

#[utoipa::path (
    post,
    tag = "Scoring",
    path = "/scoring/submit",
    responses (
        (status = Status::OK, description = "Score of the code", body = ExecutionResult),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
        (
            status = StatusCode::UNAUTHORIZED,
            description = "User's token is not authorized or missed!",
            body = ErrorResponse,
            example = json!({"status": 401, "message": "Invalid token", "details": {}})
        ),
        (status = StatusCode::REQUEST_TIMEOUT, description = "Your request timed out!", body = String, content_type = "application/json" ,example = json!("Your request timed out!")), 
    ),
    security(("jwt_token" = []))
)]
/// Submit the code and get the score
pub async fn submit(
    State(state): State<Arc<AppState>>,
    jwt_claims: JWTClaims,
    Json(data): Json<SubmitData>,
) -> Result<Json<ExecutionResult>> {
    let member = Member::get_one_by_account_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|error| Error::Unauthorized {
            message: error.to_string(),
        })?;
    let execution_result = submit_internal(state, member, data).await?;

    Ok(execution_result)
}

async fn submit_internal(
    state: Arc<AppState>,
    member: Member,
    data: SubmitData,
) -> anyhow::Result<Json<ExecutionResult>> {
    let room = Room::get_one_by_id(data.room_id, &state.database).await?;
    let now = Local::now().naive_local();
    anyhow::ensure!(room.is_open(now), "Room closed");

    let team_id = member.team_id;

    let question =
        Question::get_one_by_ids(data.question_id, room.stack_id, &state.database).await?;
    let submit_count = count_submit(data.question_id, member.id, &state.database).await?;

    anyhow::ensure!(
        submit_count < question.max_submit_time,
        "Reached the maxium number of submission(s)"
    );

    let (test_cases, template) = match room.r#type {
        RoomKind::Backend => {
            let test_cases =
                TestCase::get_many_by_question_id(data.question_id, &state.database).await?;

            (Some(test_cases), None)
        }
        RoomKind::Frontend => {
            let template =
                Template::get_one_by_question_id(data.question_id, &state.database).await?;

            (None, Some(template))
        }
    };

    let execution_result =
        util::scoring::score(data.language, &data.code, test_cases, template, 0).await?;

    save_submission(
        data.room_id,
        data.question_id,
        team_id,
        member.id,
        submit_count,
        data.language,
        data.code,
        &execution_result,
        &state.database,
    )
    .await?;

    Ok(Json(execution_result))
}

async fn count_submit(question_id: Uuid, member_id: i32, database: &PgPool) -> anyhow::Result<i32> {
    let submit_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM submit_histories WHERE question_id = $1 AND member_id = $2",
        question_id,
        member_id
    )
    .fetch_one(database)
    .await?
    .unwrap_or(0) as i32;

    Ok(submit_count)
}

async fn get_additional_penalty(
    score_id: Uuid,
    score: i32,
    database: &PgPool,
) -> anyhow::Result<i32> {
    // if pass all test cases
    if score == 0 {
        return Ok(0);
    }

    let query_result = sqlx::query!(
        r#"
        SELECT 
            SUM(CASE WHEN score > 0 THEN 1 ELSE 0 END) passed_count,
            SUM(CASE WHEN score = 0 THEN 1 ELSE 0 END) failed_count
        FROM submit_histories
        WHERE score_id = $1
        "#,
        score_id
    )
    .fetch_one(database)
    .await?;
    let passed_count = query_result.passed_count.unwrap_or(0);
    let failed_count = query_result.failed_count.unwrap_or(0);

    // current submit failed and the team had a passed submit in current room
    //   => additional penalty = 1
    // otherwise => 1 + number of failed submit in the past
    let additional_penalty = 1 + if passed_count > 0 { 0 } else { failed_count };

    Ok(additional_penalty as i32)
}

pub async fn save_submission(
    room_id: i32,
    question_id: Uuid,
    team_id: i32,
    member_id: i32,
    submit_count: i32,
    language: ProgrammingLanguage,
    code: String,
    execution_result: &ExecutionResult,
    database: &PgPool,
) -> anyhow::Result<()> {
    let now = Local::now().naive_local();

    // create new new score if not exist otherwise update
    let score_id = match Score::get_optional_by_ids(room_id, team_id, database).await? {
        None => {
            let new_score = Score {
                room_id,
                team_id,
                total_score: execution_result.score as i32,
                last_submit_time: now,
                ..Default::default()
            };
            new_score.insert(database).await?
        }
        Some(Score {
            id,
            room_id: _,
            team_id: _,
            total_score,
            last_submit_time: _,
            penalty,
        }) => {
            let additional_penalty =
                get_additional_penalty(id, execution_result.score as i32, database).await?;
            sqlx::query!(
                r#"
                UPDATE scores
                SET total_score = $2, last_submit_time = $3, penalty = $4
                WHERE id = $1
                "#,
                id,
                total_score + execution_result.score as i32,
                now,
                penalty + additional_penalty
            )
            .execute(database)
            .await?;

            id
        }
    };

    // create new submit_histories
    let new_submit_histories = SubmitHistory {
        score_id,
        question_id,
        member_id,
        submit_number: submit_count + 1,
        run_time: execution_result.run_time as i32,
        score: execution_result.score as i32,
        language,
        character_count: code.len() as i32,
        last_submit_time: now,
        submissions: code,
    };
    new_submit_histories.insert(database).await?;

    Ok(())
}
