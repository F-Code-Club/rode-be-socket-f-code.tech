use axum::{
    http::{Method, Uri},
    BoxError,
};

use crate::Error;

pub async fn timeout_handler(_method: Method, _uri: Uri, err: BoxError) -> Error {
    Error::TimedOut {
        reason: err.to_string(),
    }
}
