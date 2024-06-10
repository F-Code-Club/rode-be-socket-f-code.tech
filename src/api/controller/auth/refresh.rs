use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::config;
use crate::Result;

use super::util::TokenPair;

lazy_static! {
    static ref REFRESH_DECODING_KEY: DecodingKey =
        DecodingKey::from_secret(config::JWT_REFRESH_SECRET.as_bytes());
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshData {
    refresh_token: String,
    /// browser fingerprint
    fingerprint: String,
}

/// Generate a new token pair with extended expired time using refresh token
#[axum::debug_handler]
#[utoipa::path (
    post,
    tag = "Auth",
    path = "/auth/refresh",
    request_body = RefreshData,
    responses (
        (status = StatusCode::OK, description = "New token pair", body = TokenPair),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    )
)]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(refresh_data): Json<RefreshData>,
) -> Result<Json<TokenPair>> {
    let token_pair = refresh_internal(state, refresh_data).await?;

    Ok(token_pair)
}

async fn refresh_internal(
    state: Arc<AppState>,
    refresh_data: RefreshData,
) -> anyhow::Result<Json<TokenPair>> {
    let token_data = decode::<JWTClaims>(
        &refresh_data.refresh_token,
        &REFRESH_DECODING_KEY,
        &Validation::default(),
    )?;
    let id = token_data.claims.sub;

    let token_pair =
        TokenPair::generate(id, refresh_data.fingerprint, &state.account_fingerprints)?;

    Ok(Json(token_pair))
}
