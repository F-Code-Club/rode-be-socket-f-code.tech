use axum::{
    http::{Method, Uri},
    BoxError,
};

use crate::Error;

pub async fn handle_timeout_error(
    // `Method` and `Uri` are extractors so they can be used here
    _method: Method,
    _uri: Uri,
    // the last argument must be the error itself
    err: BoxError,
) -> Error {
    Error::TimedOut {
        reason: err.to_string(),
    }
}
