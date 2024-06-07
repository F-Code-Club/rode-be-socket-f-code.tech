use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa::ToSchema;

use super::{ErrorResponse, ErrorTrait};

#[derive(Debug, thiserror::Error)]
pub struct TimedOutError {
    message: String,
}

impl Display for TimedOutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<ErrorResponse> for TimedOutError {
    fn into(self) -> ErrorResponse {
        ErrorResponse {
            status: StatusCode::BAD_REQUEST,
            message: self.message,
            details: None,
        }
    }
}

impl ErrorTrait for TimedOutError {}
