use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub url: String,
    pub api_key: String,
    pub model: String,
    pub max_retries: u32,
    pub retry_delay_ms: u32,
}

#[derive(Debug, Deserialize)]
pub struct Providers {
    #[serde(flatten)]
    pub providers: HashMap<String, ProviderConfig>,
}