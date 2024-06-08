use once_cell::sync::Lazy;

use super::env_or_default;

pub static JWT_SECRET: Lazy<String> =
    Lazy::new(|| env_or_default("JWT_SECRET", "example".to_string()));

/// jwt expired in 1 day
pub static JWT_EXPIRED_IN: Lazy<u64> = Lazy::new(|| env_or_default("JWT_EXPIRED_IN", 24 * 60 * 60));

pub static JWT_REFRESH_SECRET: Lazy<String> =
    Lazy::new(|| env_or_default("JWT_REFRESH_SECRET", "refresh_example".to_string()));

/// jwt refresh expired in 1 week
pub static JWT_REFRESH_EXPIRED_IN: Lazy<u64> =
    Lazy::new(|| env_or_default("JWT_REFRESH_EXPIRED_IN", 7 * 24 * 60 * 60));
