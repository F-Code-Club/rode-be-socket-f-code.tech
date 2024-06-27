use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::database::model;
use crate::{Error, Result};

#[derive(Debug, Serialize, ToSchema)]
pub struct Account {
    student_id: String,
    full_name: String,
}

#[utoipa::path (
    get,
    tag = "Auth",
    path = "/auth/self",
    responses (
        (status = StatusCode::OK, body = Account),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
        (
            status = StatusCode::UNAUTHORIZED,
            description = "User's token is not authorized or missed!",
            body = ErrorResponse,
            example = json!({"status": 401, "message": "Invalid token", "details": {}})
        ),
    ),
    security(("jwt_token" = []))
)]
pub async fn me(
    State(state): State<Arc<AppState>>,
    jwt_claims: JWTClaims,
) -> Result<Json<Account>> {
    let account = model::Account::get_one_by_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|err| Error::Unauthorized {
            message: err.to_string(),
        })?;

    Ok(Json(Account {
        student_id: account.student_id,
        full_name: account.full_name,
    }))
}
