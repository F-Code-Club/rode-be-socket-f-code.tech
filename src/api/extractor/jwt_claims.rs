use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config;
use crate::Error;

lazy_static! {
    static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(config::JWT_SECRET.as_bytes());
}

#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    /// id of account in database
    pub sub: Uuid,
    pub exp: u64,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for JWTClaims {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let token = bearer.token();

        let token_data = decode::<JWTClaims>(token, &DECODING_KEY, &Validation::default())?;

        Ok(token_data.claims)
    }
}
