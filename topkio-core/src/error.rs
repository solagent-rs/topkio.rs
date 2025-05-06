use thiserror::Error;

#[derive(Error, Debug)]
pub enum TopkioError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Provider error: {0}")]
    ServerError(String),
}
