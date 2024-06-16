use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

use super::env_or_default;

pub struct KeyPair {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

static JWT_SECRET: Lazy<String> = Lazy::new(|| env_or_default("JWT_SECRET", "example".to_string()));

pub static JWT_KEYPAIR: Lazy<KeyPair> = Lazy::new(|| KeyPair {
    encoding: EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    decoding: DecodingKey::from_secret(JWT_SECRET.as_bytes()),
});

/// jwt expired in 1 day
pub static JWT_EXPIRED_IN: Lazy<u64> = Lazy::new(|| env_or_default("JWT_EXPIRED_IN", 24 * 60 * 60));

static JWT_REFRESH_SECRET: Lazy<String> =
    Lazy::new(|| env_or_default("JWT_REFRESH_SECRET", "refresh_example".to_string()));

pub static JWT_REFRESH_KEYPAIR: Lazy<KeyPair> = Lazy::new(|| KeyPair {
    encoding: EncodingKey::from_secret(JWT_REFRESH_SECRET.as_bytes()),
    decoding: DecodingKey::from_secret(JWT_REFRESH_SECRET.as_bytes()),
});

/// jwt refresh expired in 1 week
pub static JWT_REFRESH_EXPIRED_IN: Lazy<u64> =
    Lazy::new(|| env_or_default("JWT_REFRESH_EXPIRED_IN", 7 * 24 * 60 * 60));
