use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

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