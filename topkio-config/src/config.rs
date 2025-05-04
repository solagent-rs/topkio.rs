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
    #[error("Section not found: {0}")]
    SectionNotFound(String),
}

impl TopkioConfig {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Parse a specific section from the TOML file into the specified type.
    pub fn parse_section<T: for<'de> Deserialize<'de>>(path: &str, section: &str) -> Result<T, ConfigError> {
        let content = fs::read_to_string(path)?;
        let value: toml::Value = toml::from_str(&content)?;
        
        let section_value = value
            .get(section)
            .ok_or_else(|| ConfigError::SectionNotFound(section.to_string()))?;
        
        let deserialized = T::deserialize(section_value.clone())?;
        Ok(deserialized)
    }
}