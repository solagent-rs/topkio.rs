use async_trait::async_trait;
use reqwest::Client;
use topkio_core::models::{ChatCompletionRequest, ChatCompletionResponse};
use topkio_core::provider::{Provider, ProviderConfig};
use anyhow::Result;
use crate::error::ProviderError;

pub struct OllamaProvider {
    client: Client,
    config: ProviderConfig,
}

impl OllamaProvider {
    pub fn new(config: ProviderConfig) -> Self {
        OllamaProvider {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        // 示例实现，需根据 Ollama API 文档补充
        let response = self
            .client
            .post(&format!("{}/chat/completions", self.config.url))
            .json(&request)
            .send()
            .await?
            .json::<ChatCompletionResponse>()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        Ok(response)
    }
}