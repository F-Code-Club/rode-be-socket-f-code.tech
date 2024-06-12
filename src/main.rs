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
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_timer(ChronoLocal::rfc_3339()),
        )
        .init();

    // Ensure that the competition start time is initialized or return error immediately at program start
    let _ = *config::COMPETITION_START_TIME;

    let (_, _) = tokio::join!(api::start_api(), api::start_metrics());
}
