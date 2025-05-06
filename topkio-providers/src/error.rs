use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProvidersError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
    #[error("Authentication failed: {0}")]
    AuthError(String),
    #[error("Model not supported by provider: {0}")]
    UnsupportedModel(String),
    #[error("Provider-specific error: {0}")]
    ProviderError(String),
}