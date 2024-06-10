mod login;
mod refresh;

pub use login::*;
pub use refresh::*;

use chrono::Local;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::extractor::JWTClaims;
use crate::config;

lazy_static! {
    static ref REFRESH_ENCODING_KEY: EncodingKey =
        EncodingKey::from_secret(config::JWT_REFRESH_SECRET.as_bytes());
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(config::JWT_SECRET.as_bytes());
}

#[derive(Serialize, ToSchema)]
pub struct TokenPair {
    token: String,
    refresh_token: String,
}

impl TokenPair {
    fn new(id: Uuid) -> anyhow::Result<TokenPair> {
        let now = Local::now().timestamp() as u64;

        let token = encode(
            &Header::default(),
            &JWTClaims {
                sub: id,
                exp: now + *config::JWT_EXPIRED_IN,
            },
            &ENCODING_KEY,
        )?;
        let refresh_token = encode(
            &Header::default(),
            &JWTClaims {
                sub: id,
                exp: now + *config::JWT_REFRESH_EXPIRED_IN,
            },
            &REFRESH_ENCODING_KEY,
        )?;

        Ok(TokenPair {
            token,
            refresh_token,
        })
    }
}
