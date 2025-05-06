pub mod data;
pub mod gemini_impl;
use gemini_impl::chat_completion;

use topkio_core::models::Message;
use topkio_core::models::ChatCompletionResponse;
use topkio_core::backend::Backend;

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
        Ok(())
    }
}