use axum::Json;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::api::extractor::JWTClaims;
use crate::config;
use crate::Result;

use super::TokenPair;

lazy_static! {
    static ref REFRESH_DECODING_KEY: DecodingKey =
        DecodingKey::from_secret(config::JWT_REFRESH_SECRET.as_bytes());
}

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
/// Generate a new token pair with extended expired time using refresh token
pub async fn refresh(refresh_token: String) -> Result<Json<TokenPair>> {
    let token_pair = refresh_internal(refresh_token).await?;

    Ok(token_pair)
}

pub async fn refresh_internal(refresh_token: String) -> anyhow::Result<Json<TokenPair>> {
    let token_data = decode::<JWTClaims>(
        &refresh_token,
        &REFRESH_DECODING_KEY,
        &Validation::default(),
    )?;
    let id = token_data.claims.sub;

    let token_pair = TokenPair::new(id)?;

    Ok(Json(token_pair))
}
