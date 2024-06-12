use chrono_tz::Tz;
use once_cell::sync::Lazy;

use super::env_or_default;

pub const TIME_ZONE: Tz = chrono_tz::Asia::Ho_Chi_Minh;

pub static SUBMIT_TIME_OUT: Lazy<u64> = Lazy::new(|| env_or_default("SUBMIT_TIME_OUT", 5));

/// Represent the number of test cases to run when the /scoring/run is called
pub static PUBLIC_TEST_CASE_COUNT: Lazy<usize> =
    Lazy::new(|| env_or_default("PUBLIC_TEST_CASE_COUNT", 2));

// Interval between cron job of database in minute
pub static DATABASE_CRON_JOB_INTERVAL: Lazy<u64> =
    Lazy::new(|| env_or_default("DATABASE_CRON_JOB_INTERVAL", 10));
