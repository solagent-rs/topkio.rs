use serde::Deserialize;
use std::{collections::HashMap, env, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct BackendConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub supported_models: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    pub backends: HashMap<String, BackendConfig>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    FileNotFound(String),
    
    #[error("Invalid config format: {0}")]
    InvalidConfig(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),
}

impl BackendConfig {
    /// Resolves API key with environment variable fallback
    pub fn resolve_api_key(&self) -> Result<String, ConfigError> {
        if let Some(key) = &self.api_key {
            if key.starts_with("env:") {
                let var_name = &key[4..];
                env::var(var_name)
                    .map_err(|_| ConfigError::EnvVarNotFound(var_name.to_string()))
            } else {
                Ok(key.clone())
            }
        } else {
            Err(ConfigError::MissingField("api_key".to_string()))
        }
    }
}

impl GatewayConfig {
    /// Loads and validates configuration
    pub fn load_checked(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let config_str = std::fs::read_to_string(path)
            .map_err(|_| ConfigError::FileNotFound(path.display().to_string()))?;

        let mut config: GatewayConfig = toml::from_str(&config_str)
            .map_err(|e| ConfigError::InvalidConfig(e.to_string()))?;

        // Post-processing
        for (name, backend) in &mut config.backends {
            if name == "openai" {
                // Ensure OpenAI has API key
                if backend.api_key.is_none() && env::var("OPENAI_API_KEY").is_err() {
                    return Err(ConfigError::MissingField(
                        "openai.api_key or OPENAI_API_KEY".to_string(),
                    ));
                }
            }
        }

        Ok(config)
    }

    /// Gets backend config with proper error
    pub fn get_backend(&self, name: &str) -> Result<&BackendConfig, ConfigError> {
        self.backends
            .get(name)
            .ok_or_else(|| ConfigError::MissingField(format!("backends.{}", name)))
    }
}