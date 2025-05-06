use {
    crate::primitive::{ChatCompletionResponse, Message},
    anyhow::Result,
    async_trait::async_trait,
};

#[async_trait]
pub trait UnifiedLlmApi: Send + Sync {
    /// Check if the backend is healthy.
    async fn health_check(&self) -> Result<()> {
        Ok(())
    }

    /// Get the list of available models from the backend.
    async fn get_models(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    /// Request chat completion with a specific model.
    async fn chat_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        stream: Option<bool>,
    ) -> Result<ChatCompletionResponse>;

    /// Request LLM embed with a specific model.
    async fn embed(&self) -> Result<()> {
        Ok(())
    }
}
