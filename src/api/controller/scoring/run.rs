use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::database::model::{Member, Room, Template, TestCase};
use crate::enums::RoomKind;
use crate::util::{self, scoring::ExecutionResult};
use crate::{config, Error, Result};

use super::SubmitData;

#[utoipa::path (
    post,
    tag = "Scoring",
    path = "/scoring/run",
    request_body = SubmitData,
    responses (
        (status = StatusCode::OK, description = "Score of the code", body = ExecutionResult),
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
/// Run a small part of the test cases and get the result without submitting it
pub async fn run(
    State(state): State<Arc<AppState>>,
    jwt_claims: JWTClaims,
    Json(data): Json<SubmitData>,
) -> Result<Json<ExecutionResult>> {
    let _ = Member::get_one_by_account_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|error| Error::Unauthorized {
            message: error.to_string(),
        })?;

    let execution_result = run_internal(state, data).await?;

    Ok(execution_result)
}

async fn run_internal(
    state: Arc<AppState>,
    data: SubmitData,
) -> anyhow::Result<Json<ExecutionResult>> {
    let room = Room::get_one_by_id(data.room_id, &state.database).await?;
    let now = util::time::now().naive_local();
    anyhow::ensure!(room.is_open(now), "Room closed");

    let (test_cases, template) = match room.r#type {
        RoomKind::Backend => {
            let test_cases =
                TestCase::get_many_by_question_id(data.question_id, &state.database).await?;
            let public_test_cases = test_cases
                .into_iter()
                .take(*config::PUBLIC_TEST_CASE_COUNT)
                .collect::<Vec<_>>();

            (Some(public_test_cases), None)
        }
        RoomKind::Frontend => {
            let template =
                Template::get_one_by_question_id(data.question_id, &state.database).await?;

            (None, Some(template))
        }
    };

    let score = util::scoring::score(data.language, &data.code, test_cases, template, 0).await?;

    Ok(Json(score))
}
