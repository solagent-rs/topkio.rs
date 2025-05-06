use async_trait::async_trait;
use reqwest::Client;
use topkio_core::models::{ChatCompletionRequest, ChatCompletionResponse};
use topkio_core::provider::Provider;
use anyhow::Result;
use crate::error::ProvidersError;
use topkio_config::ProviderConfig;

#[derive(Clone)]
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
        let response = self
            .client
            .post(&format!("{}/api/chat", self.config.url))
            .json(&request)
            .send()
            .await?
            .json::<ChatCompletionResponse>()
            .await
            .map_err(|e| ProvidersError::InvalidResponse(e.to_string()))?;
        Ok(response)
    }
}