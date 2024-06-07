use chrono::{DateTime, TimeZone as _, Utc};
use chrono_tz::Tz;

use crate::config;

pub fn now() -> DateTime<Tz> {
    config::TIME_ZONE.from_utc_datetime(&Utc::now().naive_utc())
}
