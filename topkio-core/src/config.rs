use serde::Deserialize;
use std::fs;
use toml;
use crate::provider::ProvidersConfig;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub providers: ProvidersConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub fn load_config(path: &str) -> Result<Config, crate::error::TopkioError> {
    let content = fs::read_to_string(path)
        .map_err(|e| crate::error::TopkioError::ConfigError(e.to_string()))?;
    let config: Config = toml::from_str(&content)
        .map_err(|e| crate::error::TopkioError::ConfigError(e.to_string()))?;
    Ok(config)
}