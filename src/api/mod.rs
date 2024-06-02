use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config;
use crate::AppState;

mod controller;
mod doc;
mod extractor;
mod router;

pub async fn start_api() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    let state = Arc::new(AppState::new().await?);
    let app = router::build(state);
    let listener =
        TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), *config::SERVER_PORT)).await?;
    println!(
        "R.ODE Socket Is Started And Listening On Port: {}",
        *config::SERVER_PORT
    );
    axum::serve(listener, app).await?;

    Ok(())
}
