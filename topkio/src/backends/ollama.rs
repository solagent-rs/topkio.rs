use crate::backends::backend::Backend;
use topkio_core::models::{ChatCompletionResponse, Message};

pub struct OllamaBackend {
    base_url: String,
}

impl OllamaBackend {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[async_trait::async_trait]
impl Backend for OllamaBackend {
    async fn chat_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        stream: Option<bool>,
    ) -> Result<ChatCompletionResponse, anyhow::Error> {
        let response = ollama_client::chat_completion(
            &self.base_url,
            model,
            messages,
            stream,
        ).await?;

        Ok(response)
    }

    async fn health_check(&self) -> Result<(), anyhow::Error> {
        reqwest::get(&format!("{}/api/version", self.base_url))
            .await?
            .error_for_status()?;
        Ok(())
    }
}