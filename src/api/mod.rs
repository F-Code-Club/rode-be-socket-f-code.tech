use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;

use crate::config;
use crate::AppState;

mod controller;
mod doc;
mod extractor;
mod metrics;
mod router;

pub async fn start_api() {
    let state = Arc::new(AppState::new().await.unwrap());
    let app = router::build(state);
    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), *config::SERVER_PORT))
        .await
        .unwrap();
    tracing::info!(
        "R.ODE Socket Is Started And Listening On Port: {}",
        *config::SERVER_PORT
    );
    axum::serve(listener, app).await.unwrap();
}

pub async fn start_metrics() {
    let app = metrics::build();

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), *config::METRICS_PORT))
        .await
        .unwrap();
    tracing::debug!("Metrics server is started and listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
