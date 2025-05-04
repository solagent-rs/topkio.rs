use async_trait::async_trait;
use reqwest::Client;
use topkio_core::models::{ChatCompletionRequest, ChatCompletionResponse};
use topkio_core::provider::{Provider, ProviderConfig};
use anyhow::Result;
use crate::error::ProviderError;

pub struct OpenAIProvider {
    client: Client,
    config: ProviderConfig,
}

impl OpenAIProvider {
    pub fn new(config: ProviderConfig) -> Self {
        OpenAIProvider {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let response = self
            .client
            .post(&format!("{}/chat/completions", self.config.url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await?
            .json::<ChatCompletionResponse>()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        Ok(response)
    }
}