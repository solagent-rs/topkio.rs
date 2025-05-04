use serde::Deserialize;
use std::fs;
use thiserror::Error;

use crate::server::ServerConfig;
use crate::rate_limit::RateLimitConfig;
use crate::logging::LoggingConfig;
use crate::provider::Providers;

#[derive(Debug, Deserialize)]
pub struct TopkioConfig {
    pub server: ServerConfig,
    pub rate_limit: RateLimitConfig,
    pub logging: LoggingConfig,
    pub providers: Providers,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
}

impl TopkioConfig {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}