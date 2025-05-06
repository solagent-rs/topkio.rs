use async_trait::async_trait;
use crate::models::{Message, ChatCompletionResponse};

#[async_trait]
pub trait Backend: Send + Sync {
    async fn chat_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        stream: Option<bool>,
    ) -> Result<ChatCompletionResponse, anyhow::Error>;

    async fn health_check(&self) -> Result<(), anyhow::Error>;
}