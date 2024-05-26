use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::app_state::AppState;
use crate::Result;

use super::TokenPair;

#[derive(Deserialize, ToSchema)]
pub struct LoginData {
    email: String,
    password: String,
}

#[utoipa::path (
    post,
    tag = "Auth",
    path = "/auth/login",
    request_body = LoginData,
    responses (
        (status = StatusCode::OK, description = "Login successfully!", body = TokenPair),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(login_data): Json<LoginData>,
) -> Result<Json<TokenPair>> {
    let token_pair = login_internal(state, login_data).await?;

    Ok(token_pair)
}

pub async fn login_internal(
    state: Arc<AppState>,
    login_data: LoginData,
) -> anyhow::Result<Json<TokenPair>> {
    let id = sqlx::query_scalar!(
        "SELECT id FROM accounts WHERE email = $1 AND password = $2",
        &login_data.email,
        &login_data.password
    )
    .fetch_one(&state.database)
    .await?;

    let token_pair = TokenPair::new(id)?;

    Ok(Json(token_pair))
}
