use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, Validation};

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::{config, Result};

use super::util::TokenPair;

/// Generate a new token pair with extended expired time using refresh token
#[axum::debug_handler]
#[utoipa::path (
    post,
    tag = "Auth",
    path = "/auth/refresh",
    request_body(content = String, description = "Refresh token"),
    responses (
        (status = StatusCode::OK, description = "New token pair", body = TokenPair),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    )
)]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    refresh_token: String,
) -> Result<Json<TokenPair>> {
    let token_pair = refresh_internal(state, user_agent, refresh_token).await?;

    Ok(token_pair)
}

async fn refresh_internal(
    state: Arc<AppState>,
    user_agent: UserAgent,
    refresh_token: String,
) -> anyhow::Result<Json<TokenPair>> {
    let token_data = decode::<JWTClaims>(
        &refresh_token,
        &config::JWT_REFRESH_KEYPAIR.decoding,
        &Validation::default(),
    )?;
    let id = token_data.claims.sub;

    let token_pair = TokenPair::generate(id, user_agent.to_string(), &state.account_fingerprints)?;

    Ok(Json(token_pair))
}
