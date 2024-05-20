use axum::{
    http::{Method, StatusCode, Uri},
    BoxError,
};

pub async fn handle_timeout_error(
    // `Method` and `Uri` are extractors so they can be used here
    _method: Method,
    _uri: Uri,
    // the last argument must be the error itself
    _err: BoxError,
) -> (StatusCode, String) {
    (StatusCode::REQUEST_TIMEOUT, format!("Request timed out!"))
}
