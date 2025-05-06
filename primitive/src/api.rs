use {
    anyhow::Result,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String, // Format "backend:model_name"
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub message: Message,
}

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
