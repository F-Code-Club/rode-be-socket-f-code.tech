use once_cell::sync::Lazy;

use super::env_or_default;

pub static PUBLIC_CORS_DOMAIN: Lazy<String> =
    Lazy::new(|| env_or_default("PUBLIC_CORS_DOMAIN", "fe.domain@f-code.tech".to_string()));

pub static LOCAL_CORS_DOMAIN: Lazy<String> =
    Lazy::new(|| env_or_default("LOCAL_CORS_DOMAIN", "http://localhost:3000".to_string()));
