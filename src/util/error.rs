use std::fmt::Display;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.status, self.message)
    }
}
