#![allow(dead_code)]

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Backend not configured: {0}")]
    BackendNotConfigured(String),

    #[error("Unsupported model: {0}")]
    UnsupportedModel(String),

    #[error("Backend error: {0}")]
    BackendError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid model format: {0}")]
    InvalidModelFormat(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::BackendNotConfigured(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::UnsupportedModel(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    FileNotFound(String),

    #[error("Invalid config format: {0}")]
    InvalidConfig(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}
