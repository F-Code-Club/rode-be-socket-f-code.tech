#![feature(const_option)]
#![feature(const_int_from_str)]
#![feature(iter_array_chunks)]
#![feature(custom_test_frameworks)]

pub mod api;
pub mod app_state;
pub mod config;
pub mod database;
pub mod enums;
pub mod error;
pub mod util;

pub use error::{Error, Result};

#[macro_use]
extern crate lazy_static;

use app_state::AppState;

#[tokio::main]
async fn main() {
    let (_, _) = tokio::join!(api::start_api(), api::start_metrics());
}
