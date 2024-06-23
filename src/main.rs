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

use app_state::AppState;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_timer(fmt::time::ChronoLocal::rfc_3339()),
        )
        .with(EnvFilter::from_env("RODE_LOG"))
        .init();

    let (_, _) = tokio::join!(api::start_api(), api::start_metrics());
}
