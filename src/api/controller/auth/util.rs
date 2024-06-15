use dashmap::DashMap;

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
    pub token: String,
    pub refresh_token: String,
}

impl TokenPair {
    /// Generate new token and revoke all existing tokens associated with the account id
    pub fn generate(
        id: Uuid,
        fingerprint: String,
        account_fingerprints: &DashMap<Uuid, String>,
    ) -> anyhow::Result<TokenPair> {
        let now = Local::now().timestamp() as u64;

        // Generate tokens
        let token = encode(
            &Header::default(),
            &JWTClaims {
                sub: id,
                fingerprint: fingerprint.clone(),
                exp: now + *config::JWT_EXPIRED_IN,
            },
            &ENCODING_KEY,
        )?;
        let refresh_token = encode(
            &Header::default(),
            &JWTClaims {
                sub: id,
                fingerprint,
                exp: now + *config::JWT_REFRESH_EXPIRED_IN,
            },
            &REFRESH_ENCODING_KEY,
        )?;

        // Revoke tokens
        match account_fingerprints.get_mut(&id) {
            Some(mut token) => {
                *token = token.clone();
            }
            None => {
                account_fingerprints.insert(id, token.clone());
            }
        }

        Ok(TokenPair {
            token,
            refresh_token,
        })
    }
}
