use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::database::model::{Member, Question, Room, Score, SubmitHistory, Template, TestCase};
use crate::enums::{ProgrammingLanguage, RoomKind};
use crate::util::{self, scoring::ExecutionResult};
use crate::{config, Error, Result};

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
        (
            status = StatusCode::REQUEST_TIMEOUT,
            body = ErrorResponse,
            example = json!({"status": 408, "message": "Request timed out", "details": {}})
        ),
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

fn get_additional_penalty(is_all_passed: bool, open_time: DateTime<Tz>, now: DateTime<Tz>) -> i32 {
    if is_all_passed {
        (now - open_time).num_seconds() as i32
    } else {
        *config::FAILED_SUBMISSION_PENALTY
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn save_submission(
    room: &Room,
    question_id: Uuid,
    member: &Member,
    submit_count: i32,
    language: ProgrammingLanguage,
    code: String,
    execution_result: &ExecutionResult,
    now: DateTime<Tz>,
    database: &PgPool,
) -> anyhow::Result<()> {
    let open_time = match config::TIME_ZONE.from_local_datetime(&room.open_time) {
        chrono::offset::LocalResult::Single(open_time) => open_time,
        _ => anyhow::bail!("Invalid room open time"),
    };

    let additional_penalty = get_additional_penalty(execution_result.score > 0, open_time, now);

    let score_entry = Score::get(room.code.to_string(), member.team_id, database).await?;
    let score_id = score_entry.id;

    Score::update(score_entry, additional_penalty, now.naive_local(), database).await?;

    SubmitHistory::insert(
        question_id,
        score_id,
        member.id,
        submit_count + 1,
        execution_result.run_time as i32,
        execution_result.score as i32,
        language,
        code.len() as i32,
        now.naive_local(),
        code,
        database,
    )
    .await?;

    Ok(())
}

async fn submit_internal(
    state: Arc<AppState>,
    member: Member,
    data: SubmitData,
) -> anyhow::Result<Json<ExecutionResult>> {
    let room = Room::get_one_by_code(&data.room_code, &state.database).await?;
    let now = util::time::now();
    anyhow::ensure!(room.is_open(now.naive_local()), "Room closed");

    let question =
        Question::get_one_by_ids(data.question_id, room.stack_id, &state.database).await?;
    let submit_count =
        SubmitHistory::count(data.question_id, member.team_id, &state.database).await?;

    if data.language == ProgrammingLanguage::Css {
        anyhow::ensure!(
            submit_count < question.max_submit_time,
            "Reached the maxium number of submission(s)"
        );
    }

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
        &room,
        data.question_id,
        &member,
        submit_count,
        data.language,
        data.code,
        &execution_result,
        now,
        &state.database,
    )
    .await?;

    Ok(Json(execution_result))
}
