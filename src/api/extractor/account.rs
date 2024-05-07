use std::sync::Arc;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::config;
use crate::database::model::Account;
use crate::Error;

lazy_static! {
    static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(config::JWT_SECRET.as_bytes());
}

#[derive(Deserialize)]
pub struct JWTClaims {
    /// id of account in database
    pub sub: String,
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for Account {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let token = bearer.token();
        let token_data = decode::<JWTClaims>(token, &DECODING_KEY, &Validation::default())?;

        let account_id_raw = token_data.claims.sub;
        let account_id =
            uuid::Uuid::parse_str(&account_id_raw).map_err(|error| Error::Unauthorized {
                message: error.to_string(),
            })?;

        Account::get_one_by_id(account_id, &state.database)
            .await
            .map_err(|error| Error::Unauthorized {
                message: error.to_string(),
            })
    }
}
