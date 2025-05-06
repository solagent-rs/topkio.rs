use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,  // Format "backend:model_name"
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

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Choice {
//     pub message: Message,
// }