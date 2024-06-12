use std::env;

use chrono::{DateTime, NaiveDateTime, TimeZone as _};
use chrono_tz::Tz;
use once_cell::sync::Lazy;

use crate::util::time;

use super::env_or_default;

pub const TIME_ZONE: Tz = chrono_tz::Asia::Ho_Chi_Minh;

pub static FAILED_SUBMISSION_PENALTY: Lazy<i32> =
    Lazy::new(|| env_or_default("FAILED_SUBMISSION_PENALTY", 13));

/// Start time of the competition in ISO 8601 format
///
/// # Panics
/// This will panic if
/// - The environment variable is not a valid ISO 8601 date
/// - The specified start time is in the past
pub static COMPETITION_START_TIME: Lazy<DateTime<Tz>> = Lazy::new(|| {
    let date_time_raw = env::var("COMPETITION_START_TIME").unwrap();
    let naive_date_time = NaiveDateTime::parse_from_str(&date_time_raw, "%+").unwrap();
    let start_time = TIME_ZONE.from_local_datetime(&naive_date_time).unwrap();
    assert!(start_time > time::now());

    start_time
});

pub static SUBMIT_TIME_OUT: Lazy<u64> = Lazy::new(|| env_or_default("SUBMIT_TIME_OUT", 5));

/// Represent the number of test cases to run when the /scoring/run is called
pub static PUBLIC_TEST_CASE_COUNT: Lazy<usize> =
    Lazy::new(|| env_or_default("PUBLIC_TEST_CASE_COUNT", 2));
