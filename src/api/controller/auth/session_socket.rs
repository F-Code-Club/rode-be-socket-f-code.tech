use std::sync::Arc;

use anyhow::Context;
use axum::extract::ws::Message;
use axum::extract::{ws::WebSocket, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::StreamExt;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::config;

lazy_static! {
    static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(config::JWT_SECRET.as_bytes());
}

#[utoipa::path (
    get,
    tag = "Auth",
    path = "/auth/session/socket",
    responses (
        (status = 101, description = "Connect to websocket successfully!"),
        (status = 1006, description = "Connection is closed abnormally!"),
        (status = 400, description = "Bad request!")
    )
)]

pub async fn session_socket(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        let _ = session_socket_internal(state, socket).await;
    })
}

#[tracing::instrument(err)]
async fn session_socket_internal(state: Arc<AppState>, stream: WebSocket) -> anyhow::Result<()> {
    let (_, mut receiver) = stream.split();

    // get account id from token
    let message = receiver.next().await.context("No message found")??;
    let Message::Text(token) = message else {
        anyhow::bail!("Expect a token")
    };
    let token_data = decode::<JWTClaims>(&token, &DECODING_KEY, &Validation::default())?;
    let id = token_data.claims.sub;

    tracing::info!("New id joined: {}", id);

    // Add id
    match state.logged_in_account_ids.lock() {
        Ok(mut logged_in_account_ids) => {
            if logged_in_account_ids.contains(&id) {
                tracing::info!("{} already logged in", id);
                return Ok(());
            }
            logged_in_account_ids.insert(id);
        }
        Err(error) => {
            anyhow::bail!(error.to_string())
        }
    };
    tracing::info!("Added {}", id);

    // task created to keep the connection alive
    let dummy_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            tracing::info!(text);
        }
    });
    let _ = dummy_task.await;

    // Remove id
    match state.logged_in_account_ids.lock() {
        Ok(mut logged_in_account_ids) => {
            logged_in_account_ids.remove(&id);
        }
        Err(error) => {
            anyhow::bail!(error.to_string())
        }
    };
    tracing::info!("{} logout", id);

    Ok(())
}
