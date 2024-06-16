use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::extract::{
    ws::{Message, WebSocket},
    State, WebSocketUpgrade,
};
use axum::response::IntoResponse;
use futures::stream::{SplitSink, SplitStream};
use futures::{Stream, StreamExt};
use tokio::sync::{Mutex, RwLock};
use yrs::{sync::Awareness, Doc};
use yrs_warp::{broadcast::BroadcastGroup, AwarenessRef};

use crate::app_state::AppState;
use crate::Error;

#[utoipa::path (
    get,
    tag = "Editor",
    path = "/editor/socket/{question_id}/{team_id}",
    params(
        ("question_id" = String, Path, description = "Question id"),
        ("team_id" = String, Path, description = "Team id")
    ),
)]
/// A web socket endpoint to broadcast CRDT event (insert, delete, ...) between several editor instance
pub async fn editor_socket(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        let _ = editor_socket_internal(socket, state).await;
    })
}

/// Adapted from [yrs_warp::ws::WarpSink]
#[repr(transparent)]
#[derive(Debug)]
struct AxumSink(SplitSink<WebSocket, Message>);

impl From<SplitSink<WebSocket, Message>> for AxumSink {
    fn from(sink: SplitSink<WebSocket, Message>) -> Self {
        AxumSink(sink)
    }
}

impl Into<SplitSink<WebSocket, Message>> for AxumSink {
    fn into(self) -> SplitSink<WebSocket, Message> {
        self.0
    }
}

impl futures_util::Sink<Vec<u8>> for AxumSink {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match Pin::new(&mut self.0).poll_ready(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(Error::Other(anyhow::Error::from(e)))),
            Poll::Ready(_) => Poll::Ready(Ok(())),
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: Vec<u8>) -> Result<(), Self::Error> {
        Pin::new(&mut self.0)
            .start_send(Message::Binary(item))
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match Pin::new(&mut self.0).poll_flush(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(Error::Other(e.into()))),
            Poll::Ready(_) => Poll::Ready(Ok(())),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match Pin::new(&mut self.0).poll_close(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(Error::Other(e.into()))),
            Poll::Ready(_) => Poll::Ready(Ok(())),
        }
    }
}

/// Adapted from [yrs_warp::ws::WarpStream]
#[derive(Debug)]
struct AxumStream(SplitStream<WebSocket>);

impl From<SplitStream<WebSocket>> for AxumStream {
    fn from(stream: SplitStream<WebSocket>) -> Self {
        AxumStream(stream)
    }
}

impl Into<SplitStream<WebSocket>> for AxumStream {
    fn into(self) -> SplitStream<WebSocket> {
        self.0
    }
}

impl Stream for AxumStream {
    type Item = Result<Vec<u8>, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(res)) => match res {
                Ok(item) => {
                    if let Message::Binary(binaries) = item {
                        Poll::Ready(Some(Ok(binaries)))
                    } else {
                        Poll::Ready(Some(Err(Error::Other(anyhow::anyhow!("Error")))))
                    }
                }
                Err(e) => Poll::Ready(Some(Err(Error::Other(e.into())))),
            },
        }
    }
}

#[tracing::instrument(level = "error", skip(stream))]
async fn editor_socket_internal(stream: WebSocket, state: Arc<AppState>) -> anyhow::Result<()> {
    let (sink, stream) = stream.split();

    let sink = Arc::new(Mutex::new(AxumSink::from(sink)));
    let stream = AxumStream::from(stream);

    let awareness: AwarenessRef = {
        let doc = Doc::new();
        Arc::new(RwLock::new(Awareness::new(doc)))
    };

    let broadcast_group = BroadcastGroup::new(awareness.clone(), 32).await;

    let sub = broadcast_group.subscribe(sink, stream);

    sub.completed().await?;

    Ok(())
}
