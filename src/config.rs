use std::{env, str::FromStr};

use once_cell::sync::Lazy;

fn env_or_default<T: FromStr>(env_name: &'static str, default: T) -> T {
    match env::var(env_name) {
        Err(_) => default,
        Ok(raw) => match raw.parse() {
            Ok(value) => value,
            Err(_) => default,
        },
    }
}

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| env_or_default("DATABASE_URL", "postgres://user:password@host/database".to_string()));

pub static SERVER_PORT: Lazy<u16> = Lazy::new(|| env_or_default("SERVER_PORT", 3000));

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| env_or_default("JWT_SECRET", "example".to_string()));

pub static PUBLIC_CORS_DOMAIN: Lazy<String> = Lazy::new(|| env_or_default("PUBLIC_CORS_DOMAIN", "fe.domain@f-code.tech".to_string()));

pub static LOCAL_CORS_DOMAIN: Lazy<String> = Lazy::new(|| env_or_default("LOCAL_CORS_DOMAIN", "localhost:3000".to_string()));
/// jwt expired in 1 day
pub static JWT_EXPIRED_IN: Lazy<u64> = Lazy::new(|| env_or_default("JWT_EXPIRED_IN", 24 * 60 * 60));
pub static JWT_REFRESH_SECRET: Lazy<String> = Lazy::new(|| env_or_default("JWT_REFRESH_SECRET", "refresh_example".to_string()));
/// jwt refresh expired in 1 week
pub static JWT_REFRESH_EXPIRED_IN: Lazy<u64> = Lazy::new(|| env_or_default("JWT_REFRESH_EXPIRED_IN", 7 * 24 * 60 * 60));

/// Represent the number of test cases to run when the /scoring/run is called
pub static PUBLIC_TEST_CASE_COUNT: Lazy<usize> = Lazy::new(|| env_or_default("PUBLIC_TEST_CASE_COUNT", 2));

pub static GOOGLE_CLIENT_ID: Lazy<String> = Lazy::new(|| env_or_default("GOOGLE_CLIENT_ID", "".to_string()));
pub static GOOGLE_CLIENT_SECRET: Lazy<String> = Lazy::new(|| env_or_default("GOOGLE_CLIENT_SECRET", "".to_string()));
pub static GOOGLE_REDIRECT_URL: Lazy<String> = Lazy::new(|| env_or_default("GOOGLE_REDIRECT_URL", "".to_string()));
pub static GOOGLE_REFRESH_TOKEN: Lazy<String> = Lazy::new(|| env_or_default("GOOGLE_REFRESH_TOKEN", "".to_string()));
