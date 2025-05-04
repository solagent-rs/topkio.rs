use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub providers: ProvidersConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ProvidersConfig {
    pub openai_enabled: bool,
    pub anthropic_enabled: bool,
}

impl Config {
    pub fn load() -> Self {
        let config_path = "config/gateway.toml";
        let config_contents = std::fs::read_to_string(config_path)
            .expect("Failed to read config file");
        
        let config: Config = toml::from_str(&config_contents)
            .expect("Invalid config format");

        // No secrets in the config file - they come from .env
        config
    }

    pub fn openai_key() -> String {
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")
    }

    pub fn anthropic_key() -> String {
        env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set")
    }
}