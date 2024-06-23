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
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_timer(fmt::time::ChronoLocal::rfc_3339()),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .with_env_var("RODE_LOG")
                .from_env_lossy(),
        )
        .init();

    let (_, _) = tokio::join!(api::start_server(), api::start_metrics());
}
