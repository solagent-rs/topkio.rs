use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    // Additional common parameters
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub text: String,
    pub model: String,
    pub usage: UsageStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageStats {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug)]
pub enum GatewayError {
    RateLimitExceeded,
    InvalidRequest,
    ProviderError(String),
}