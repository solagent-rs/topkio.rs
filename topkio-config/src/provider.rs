use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderConfig {
    pub url: String,
    pub api_key: String,
    pub max_retries: u32,
    pub retry_delay_ms: u32,
    pub models: HashMap<String, ModelConfig>, // Key: 模型名（如 "llama3"）
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModelConfig {
    pub max_tokens: u32,
    pub temperature_range: [f32; 2], // 温度范围 [min, max]
}