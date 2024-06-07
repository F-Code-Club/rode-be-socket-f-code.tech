use axum::{
    http::{Method, Uri},
    BoxError,
};

use crate::Error;

pub async fn handle_timeout_error(_method: Method, _uri: Uri, err: BoxError) -> Error {
    Error::TimedOut {
        message: err.to_string(),
    }
}
