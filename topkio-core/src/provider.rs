use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::models::{ChatCompletionRequest, ChatCompletionResponse};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub url: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvidersConfig {
    pub openai: Option<ProviderConfig>,
    pub gemini: Option<ProviderConfig>,
    pub ollama: Option<ProviderConfig>,
    pub deepseek: Option<ProviderConfig>,
}

#[async_trait]
pub trait Provider : Clone {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse>;
}