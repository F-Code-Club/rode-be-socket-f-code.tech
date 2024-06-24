use once_cell::sync::Lazy;

use super::env_or_default;

pub static GOOGLE_CLIENT_EMAIL: Lazy<String> =
    Lazy::new(|| env_or_default("GOOGLE_CLIENT_EMAIL", "".to_string()));

pub static GOOGLE_PRIVATE_KEY: Lazy<String> =
    Lazy::new(|| env_or_default("GOOGLE_PRIVATE_KEY", "".to_string()));

pub static GOOGLE_PRIVATE_KEY_ID: Lazy<String> =
    Lazy::new(|| env_or_default("GOOGLE_PRIVATE_KEY_ID", "".to_string()));
