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

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    FileNotFound(String),

    #[error("Invalid config format: {0}")]
    InvalidConfig(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}
