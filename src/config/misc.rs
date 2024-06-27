use std::path::PathBuf;

use chrono_tz::Tz;
use once_cell::sync::{Lazy, OnceCell};

use super::env_or_default;

pub const TIME_ZONE: Tz = chrono_tz::Asia::Ho_Chi_Minh;

pub static SUBMIT_TIME_OUT: Lazy<u64> = Lazy::new(|| env_or_default("SUBMIT_TIME_OUT", 5));

pub static TEMPLATE_PATH: Lazy<PathBuf> =
    Lazy::new(|| env_or_default("TEMPLATE_PATH", PathBuf::from("/tmp/template")));

/// Time out for each test case in second
pub static TEST_CASE_TIME_OUT: Lazy<u32> = Lazy::new(|| env_or_default("TEST_CASE_TIME_OUT", 3));

pub static CHROME_PATH: OnceCell<PathBuf> = OnceCell::new();
