use crate::backends::backend::Backend;
use topkio_core::models::{ChatCompletionResponse, Message};
use gemini_client::{chat_completion, GeminiResponse};

pub struct GeminiBackend {
    base_url: String,
    api_key: String,
}

impl GeminiBackend {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self { base_url, api_key }
    }
}

#[async_trait::async_trait]
impl Backend for GeminiBackend {
    async fn chat_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        stream: Option<bool>,
    ) -> Result<ChatCompletionResponse, anyhow::Error> {
        let response = chat_completion(
            &self.base_url,
            &self.api_key,
            model,
            messages,
        ).await?;

        // Convert Gemini response to our standard format
        let first_candidate = response.candidates.first()
            .ok_or_else(|| anyhow::anyhow!("No candidates in response"))?;
        
        let first_part = first_candidate.content.parts.first()
            .ok_or_else(|| anyhow::anyhow!("No content parts in response"))?;

        // Ok(ChatCompletionResponse {
        //     message: Message {
        //         role: first_candidate.content.role.clone().unwrap_or("model".to_string()),
        //         content: first_part.text.clone(),
        //     },
        // })

        todo!("Convert Gemini response to standard format");
    }

    async fn health_check(&self) -> Result<(), anyhow::Error> {
        // Simple health check by requesting model list
        let url = format!("{}/models?key={}", self.base_url, self.api_key);
        reqwest::get(&url)
            .await?
            .error_for_status()?;
        Ok(())
    }
}