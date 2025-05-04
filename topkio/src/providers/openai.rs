use reqwest::Client;
use serde_json::json;

use crate::{
    config::ProvidersConfig,
    models::{CompletionRequest, CompletionResponse, UsageStats},
};

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new(config: ProvidersConfig) -> Self {
        Self {
            client: Client::new(),
            api_key: config.api_key,
            base_url: config.base_url,
        }
    }
}

#[async_trait::async_trait]
impl super::Provider for OpenAIProvider {
    async fn create_completion(
        &self,
        request: crate::models::CompletionRequest,
    ) -> Result<CompletionResponse, String> {
        let response = self
            .client
            .post(&format!("{}/v1/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": request.model,
                "prompt": request.prompt,
                "max_tokens": request.max_tokens,
                "temperature": request.temperature,
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| e.to_string())?;

        Ok(CompletionResponse {
            text: response_json["choices"][0]["text"].as_str().unwrap_or_default().to_string(),
            model: request.model,
            usage: UsageStats {
                prompt_tokens: response_json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
                completion_tokens: response_json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
                total_tokens: response_json["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32,
            },
        })
    }
}