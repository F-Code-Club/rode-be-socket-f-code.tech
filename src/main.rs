#![feature(custom_test_frameworks)]

pub mod api;
pub mod app_state;
pub mod config;
pub mod database;
pub mod enums;
pub mod error;
pub mod util;

use std::path::Path;

use chromiumoxide::{BrowserFetcher, BrowserFetcherOptions};
pub use error::{Error, Result};

use app_state::AppState;
use tokio::fs;
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

    let download_path = Path::new("./download");
    fs::create_dir_all(&download_path).await.unwrap();
    let fetcher = BrowserFetcher::new(
        BrowserFetcherOptions::builder()
            .with_path(download_path)
            .build()
            .unwrap(),
    );
    let info = fetcher.fetch().await.unwrap();
    let chrome_path = info.executable_path;

    config::CHROME_PATH.set(chrome_path).unwrap();

    let (_, _) = tokio::join!(api::start_server(), api::start_metrics());
}
