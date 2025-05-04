use async_trait::async_trait;
use reqwest::Client;
use topkio_core::models::{ChatCompletionRequest, ChatCompletionResponse};
use topkio_core::provider::{Provider, ProviderConfig};
use anyhow::Result;
use crate::error::ProviderError;

#[derive(Clone)]
pub struct GeminiProvider {
    client: Client,
    config: ProviderConfig,
}

impl GeminiProvider {
    pub fn new(config: ProviderConfig) -> Self {
        GeminiProvider {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        // 示例实现，需根据 Google Gemini API 文档补充
        let response = self
            .client
            .post(&format!("{}/models/{}:generateContent", self.config.url, request.model))
            .query(&[("key", &self.config.api_key)])
            .json(&request)
            .send()
            .await?
            .json::<ChatCompletionResponse>()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        Ok(response)
    }
}