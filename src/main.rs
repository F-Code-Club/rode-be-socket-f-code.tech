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

use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use app_state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    let state = Arc::new(AppState::new().await?);
    let app = api::router::build(state);
    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), config::PORT)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
