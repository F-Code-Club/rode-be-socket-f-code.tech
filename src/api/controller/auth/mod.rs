mod login;
mod refresh;
mod session_socket;

pub use login::*;
pub use refresh::*;
pub use session_socket::*;

use chrono::Local;
use jsonwebtoken::{encode, Header};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::extractor::JWTClaims;
use crate::config;

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
            &config::JWT_KEYPAIR.encoding,
        )?;
        let refresh_token = encode(
            &Header::default(),
            &JWTClaims {
                sub: id,
                exp: now + *config::JWT_REFRESH_EXPIRED_IN,
            },
            &config::JWT_REFRESH_KEYPAIR.encoding,
        )?;

        Ok(TokenPair {
            token,
            refresh_token,
        })
    }
}
