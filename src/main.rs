pub mod router;
pub mod controller;
pub mod middleware;
pub mod util;
pub mod model;

#[macro_use]
extern crate lazy_static;
use anyhow::Result;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use util::{app_state::AppState, keys::Keys};
use dotenvy_macro::dotenv;

lazy_static! {
    static ref KEYS: Keys = Keys::new(dotenv!("JWT_SECRET").as_bytes());
}


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    let state = AppState::new().await?;
    let app = router::build(state);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
