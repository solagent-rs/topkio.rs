use async_trait::async_trait;
use crate::models::ChatCompletionResponse;
use ollama_client::ChatMessage;

#[async_trait]
pub trait Backend: Send + Sync {
    async fn chat_completion(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<ChatCompletionResponse, anyhow::Error>;

    async fn health_check(&self) -> Result<(), anyhow::Error>;
}