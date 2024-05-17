use std::sync::Arc;

use axum::Json;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::Result;

use super::TokenPair;

#[derive(Deserialize)]
pub struct LoginData {
    email: String,
    password: String,
}

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
