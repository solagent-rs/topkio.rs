use {
    crate::ollama::chat_completion::chat_completion,
    topkio_core::{
        api::UnifiedLlmApi,
        primitive::{ChatCompletionResponse, Message},
    },
};

pub struct OllamaBackend {
    base_url: String,
}

impl OllamaBackend {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[async_trait::async_trait]
impl UnifiedLlmApi for OllamaBackend {
    async fn chat_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        stream: Option<bool>,
    ) -> Result<ChatCompletionResponse, anyhow::Error> {
        let response = chat_completion(&self.base_url, model, messages, stream).await?;

        Ok(response)
    }

    async fn health_check(&self) -> Result<(), anyhow::Error> {
        reqwest::get(&format!("{}/api/version", self.base_url))
            .await?
            .error_for_status()?;
        Ok(())
    }
}
