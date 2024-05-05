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

pub const DATABASE_URL: &str = env_or_default!("DATABASE_URL", "mysql://user:pass@host/database");
pub const JWT_SECRET: &str = env_or_default!("JWT_SECRET", "example");
pub const PORT: u16 = env_as_number_or_default!(u16, "PORT", 3000);
pub const FILE_COUNT_LIMIT: usize = env_as_number_or_default!(usize, "FILE_COUNT_LIMIT", 5);
/// Upload file size limit in byte
pub const FILE_SIZE_LIMIT: usize = env_as_number_or_default!(usize, "FILE_SIZE_LIMIT", 1024 * 1024);
pub const UPLOAD_LOCATION: &str = env_or_default!("UPLOAD_LOCATION", "uploads/question-files");