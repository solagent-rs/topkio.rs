use serde::{Deserialize, Serialize};
use ollama_client::ChatMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,  // Format "backend:model_name"
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionResponse {
    pub message: ChatMessage,
}