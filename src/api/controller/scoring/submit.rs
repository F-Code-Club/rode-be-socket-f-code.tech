use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use chrono::{DateTime, TimeZone as _};
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

fn get_additional_penalty(is_passed: bool, open_time: DateTime<Tz>, now: DateTime<Tz>) -> i32 {
    if is_passed {
        (now - open_time).num_seconds() as i32
    } else {
        *config::FAILED_SUBMISSION_PENALTY
    }
}

async fn execute(
    room: &Room,
    question_id: Uuid,
    language: ProgrammingLanguage,
    code: &str,
    database: &PgPool,
) -> anyhow::Result<ExecutionResult> {
    let (test_cases, template) = match room.r#type {
        RoomKind::Backend => {
            let test_cases = TestCase::get_many_by_question_id(question_id, database).await?;

            (Some(test_cases), None)
        }
        RoomKind::Frontend => {
            let template = Template::get_one_by_question_id(question_id, database).await?;

            (None, Some(template))
        }
    };

    let execution_result = util::scoring::score(language, code, test_cases, template, 0).await?;

    Ok(execution_result)
}

async fn save_submission(
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

    let score_id = Score::update_or_insert(
        room.id,
        member.team_id,
        additional_penalty,
        now.naive_local(),
        database,
    )
    .await?;

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
    let database = &state.database;
    let SubmitData {
        room_id,
        question_id,
        language,
        code,
    } = data;
    let now = util::time::now();

    let room = Room::get_one_by_id(room_id, &state.database).await?;
    anyhow::ensure!(room.is_open(now.naive_local()), "Room closed");

    let question = Question::get_one_by_ids(question_id, room.stack_id, database).await?;
    let submit_count = SubmitHistory::count(question_id, member.id, database).await? as i32;
    anyhow::ensure!(
        submit_count < question.max_submit_time,
        "Reached the maxium number of submission(s)"
    );

    let execution_result = execute(&room, question_id, language, &code, database).await?;

    save_submission(
        &room,
        question_id,
        &member,
        submit_count,
        language,
        code,
        &execution_result,
        now,
        database,
    )
    .await?;

    Ok(Json(execution_result))
}
