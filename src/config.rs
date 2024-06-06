use std::{env, str::FromStr};

use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
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

pub const TIME_ZONE: Tz = chrono_tz::Asia::Ho_Chi_Minh;

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    env_or_default(
        "DATABASE_URL",
        "postgres://user:password@host/database".to_string(),
    )
});

pub static SERVER_PORT: Lazy<u16> = Lazy::new(|| env_or_default("SERVER_PORT", 3000));
pub static METRICS_PORT: Lazy<u16> = Lazy::new(|| env_or_default("METRICS_PORT", 3001));

pub static JWT_SECRET: Lazy<String> =
    Lazy::new(|| env_or_default("JWT_SECRET", "example".to_string()));

pub static PUBLIC_CORS_DOMAIN: Lazy<String> =
    Lazy::new(|| env_or_default("PUBLIC_CORS_DOMAIN", "fe.domain@f-code.tech".to_string()));

pub static LOCAL_CORS_DOMAIN: Lazy<String> =
    Lazy::new(|| env_or_default("LOCAL_CORS_DOMAIN", "http://localhost:3000".to_string()));

pub static SUBMIT_TIME_OUT: Lazy<u64> = Lazy::new(|| env_or_default("SUBMIT_TIME_OUT", 5));

/// jwt expired in 1 day
pub static JWT_EXPIRED_IN: Lazy<u64> = Lazy::new(|| env_or_default("JWT_EXPIRED_IN", 24 * 60 * 60));
pub static JWT_REFRESH_SECRET: Lazy<String> =
    Lazy::new(|| env_or_default("JWT_REFRESH_SECRET", "refresh_example".to_string()));
/// jwt refresh expired in 1 week
pub static JWT_REFRESH_EXPIRED_IN: Lazy<u64> =
    Lazy::new(|| env_or_default("JWT_REFRESH_EXPIRED_IN", 7 * 24 * 60 * 60));

/// Represent the number of test cases to run when the /scoring/run is called
pub static PUBLIC_TEST_CASE_COUNT: Lazy<usize> =
    Lazy::new(|| env_or_default("PUBLIC_TEST_CASE_COUNT", 2));

pub static GOOGLE_CLIENT_EMAIL: Lazy<String> =
    Lazy::new(|| env_or_default("GOOGLE_CLIENT_EMAIL", "".to_string()));
pub static GOOGLE_PRIVATE_KEY: Lazy<String> =
    Lazy::new(|| env_or_default("GOOGLE_PRIVATE_KEY", "".to_string()));
pub static GOOGLE_PRIVATE_KEY_ID: Lazy<String> =
    Lazy::new(|| env_or_default("GOOGLE_PRIVATE_KEY_ID", "".to_string()));
