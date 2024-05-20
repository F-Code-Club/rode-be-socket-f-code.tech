use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::database::model::Member;
use crate::{Error, Result};

#[utoipa::path (
    get,
    tag = "Team",
    path = "/team/get-id",
    responses (
        (status = Status::OK, description = "Member's team id", body = i32),
        (
            status = StatusCode::UNAUTHORIZED,
            description = "User's token is not authorized or missed!",
            body = ErrorResponse,
            example = json!({"status": 401, "message": "Invalid token", "details": {}})
        )
    ),
    security(("jwt_token" = []))
)]
/// Get team id of current logged in member
pub async fn get_id(
    State(state): State<Arc<AppState>>,
    jwt_claims: JWTClaims,
) -> Result<Json<i32>> {
    let member = Member::get_one_by_account_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|error| Error::Unauthorized {
            message: error.to_string(),
        })?;

    Ok(Json(member.team_id))
}
