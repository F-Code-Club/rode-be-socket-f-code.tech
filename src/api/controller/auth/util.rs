use dashmap::DashMap;

use chrono::Local;
use jsonwebtoken::{encode, Header};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::extractor::JWTClaims;
use crate::config;

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
            &config::JWT_KEYPAIR.encoding,
        )?;
        let refresh_token = encode(
            &Header::default(),
            &JWTClaims {
                sub: id,
                fingerprint,
                exp: now + *config::JWT_REFRESH_EXPIRED_IN,
            },
            &config::JWT_REFRESH_KEYPAIR.encoding,
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
