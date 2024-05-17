use std::sync::Arc;

use axum::Json;
use axum::{extract::State, Form};
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
        (status = 200, description = "Login successfully!",body = TokenPair),
        (status = 400, description = "Bad request!"),
        (status = 401, description = "User's token pair is not authorized or missed!")
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Form(login_data): Form<LoginData>,
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
