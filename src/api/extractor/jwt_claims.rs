use std::sync::Arc;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::UserAgent;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::config;
use crate::Error;

#[derive(Clone, Serialize, Deserialize)]
pub struct JWTClaims {
    /// id of account in database
    pub sub: Uuid,
    pub fingerprint: String,
    pub exp: u64,
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for JWTClaims {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let token = bearer.token();

        let token_data =
            decode::<JWTClaims>(token, &config::JWT_KEYPAIR.decoding, &Validation::default())?;

        let claims = token_data.claims.clone();
        let account_id = claims.sub;

        let fingerprint = claims.fingerprint;
        let TypedHeader(user_agent) = parts.extract::<TypedHeader<UserAgent>>().await?;
        if fingerprint != user_agent.to_string() {
            return Err(Error::Unauthorized {
                message: "Invalid token".to_string(),
            });
        }

        // Ensure that only the latest logged in device can process further
        let is_valid_fingerprint = match state.account_fingerprints.get(&account_id) {
            None => true,
            Some(valid_fingerprint) => fingerprint == *valid_fingerprint.value(),
        };
        if !is_valid_fingerprint {
            return Err(Error::Unauthorized {
                message: "Invalid token".to_string(),
            });
        }

        Ok(token_data.claims)
    }
}
