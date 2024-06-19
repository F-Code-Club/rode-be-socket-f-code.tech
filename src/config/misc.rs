use chrono_tz::Tz;
use once_cell::sync::Lazy;

use super::env_or_default;

pub const TIME_ZONE: Tz = chrono_tz::Asia::Ho_Chi_Minh;

pub static SUBMIT_TIME_OUT: Lazy<u64> = Lazy::new(|| env_or_default("SUBMIT_TIME_OUT", 5));
