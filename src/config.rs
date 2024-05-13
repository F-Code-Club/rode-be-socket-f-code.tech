macro_rules! env_or_default {
    ($env_name:literal, $default:expr) => {
        match option_env!($env_name) {
            Some(value) => value,
            None => $default,
        }
    };
}
macro_rules! env_as_number_or_default {
    ($number_type:ty, $env_name:literal, $default:expr) => {
        match option_env!($env_name) {
            #[allow(clippy::from_str_radix_10)]
            Some(value) => match <$number_type>::from_str_radix(value, 10) {
                Ok(value) => value,
                Err(_) => $default,
            },
            None => $default,
        }
    };
}

pub const DATABASE_URL: &str = env_or_default!("DATABASE_URL", "postgres://user:password@host/database");
/// Server port
pub const PORT: u16 = env_as_number_or_default!(u16, "PORT", 3000);

pub const JWT_SECRET: &str = env_or_default!("JWT_SECRET", "example");
/// jwt expired in 1 day
pub const JWT_EXPIRED_IN: u64 = env_as_number_or_default!(u64, "JWT_EXPIRED_IN", 24 * 60 * 60);
pub const JWT_REFRESH_SECRET: &str = env_or_default!("JWT_REFRESH_SECRET", "refresh_example");
/// jwt refresh expired in 1 week
pub const JWT_REFRESH_EXPIRED_IN: u64 = env_as_number_or_default!(u64, "JWT_REFRESH_EXPIRED_IN", 7 * 24 * 60 * 60);

/// Represent the number of test cases to run when the /scoring/run is called
pub const PUBLIC_TEST_CASE_COUNT: usize =
    env_as_number_or_default!(usize, "PUBLIC_TEST_CASE_COUNT", 2);

pub const GOOGLE_CLIENT_ID: &str = env_or_default!("GOOGLE_CLIENT_ID", "");
pub const GOOGLE_CLIENT_SECRET: &str = env_or_default!("GOOGLE_CLIENT_SECRET", "");
pub const GOOGLE_REDIRECT_URL: &str = env_or_default!("GOOGLE_REDIRECT_URL", "");
pub const GOOGLE_REFRESH_TOKEN: &str = env_or_default!("GOOGLE_REFRESH_TOKEN", "");
