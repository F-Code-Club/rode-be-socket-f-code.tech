use once_cell::sync::Lazy;

use super::env_or_default;

pub static GOOGLE_API_KEY_PATH: Lazy<String> =
    Lazy::new(|| env_or_default("GOOGLE_API_KEY_PATH", "".to_string()));
