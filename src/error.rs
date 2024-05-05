use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::{collections::HashMap, fmt::Display};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Unauthorized { message: String },
    Other(anyhow::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message, details) = match self {
            Error::Unauthorized { message } => (
                StatusCode::UNAUTHORIZED,
                message,
                HashMap::<String, String>::new(),
            ),
            Error::Other(error) => (
                StatusCode::BAD_REQUEST,
                error.to_string(),
                HashMap::<String, String>::new(),
            ),
        };
        let response = Json(json!({
            "status": status.as_u16(),
            "message": message,
            "details": details
        }));
        (status, response).into_response()
    }
}

impl From<axum_extra::typed_header::TypedHeaderRejection> for Error {
    fn from(value: axum_extra::typed_header::TypedHeaderRejection) -> Self {
        Error::Unauthorized { message: value.to_string() }
    }
}
impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Error::Unauthorized { message: value.to_string() }
    }
}
impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error::Other(value)
    }
}