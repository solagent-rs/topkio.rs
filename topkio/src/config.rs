use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf, time::Duration};
use crate::error::ConfigError;

#[derive(Debug, Deserialize)]
pub struct GatewayConfig {
    pub server: ServerConfig,
    pub rate_limit: Option<RateLimitConfig>,
    pub logging: LoggingConfig,
    pub providers: ProvidersConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub requests_per_minute: u32,
    #[serde(default = "default_burst_size")]
    pub burst_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    pub file_path: PathBuf,
    #[serde(default = "default_console_logging")]
    pub enable_console: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProvidersConfig {
    pub openai: Option<ProviderConfig>,
    pub gemini: Option<ProviderConfig>,
    pub ollama: Option<ProviderConfig>,
    pub deepseek: Option<ProviderConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub supported_models: Vec<String>,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_retry_delay_ms")]
    pub retry_delay_ms: u64,
}

// Default values
fn default_timeout() -> u64 { 30 }
fn default_max_connections() -> u32 { 1000 }
fn default_enabled() -> bool { true }
fn default_burst_size() -> u32 { 10 }
fn default_log_level() -> String { "info".into() }
fn default_console_logging() -> bool { true }
fn default_max_retries() -> u32 { 3 }
fn default_retry_delay_ms() -> u64 { 500 }

impl GatewayConfig {
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        let config_str = std::fs::read_to_string(path)
            .map_err(|_| ConfigError::FileNotFound(path.into()))?;

        let config: GatewayConfig = toml::from_str(&config_str)
            .map_err(|e| ConfigError::InvalidConfig(e.to_string()))?;

        // Post-load validation
        if let Some(openai) = &config.providers.openai {
            if openai.api_key.is_none() && std::env::var("OPENAI_API_KEY").is_err() {
                return Err(ConfigError::MissingField(
                    "providers.openai.api_key or OPENAI_API_KEY".into()
                ));
            }
        }

        Ok(config)
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Invalid server address")
    }
}