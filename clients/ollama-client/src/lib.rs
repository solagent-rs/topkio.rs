use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
}

pub async fn chat_completion(
    base_url: &str,
    model: &str,
    messages: Vec<ChatMessage>,
) -> Result<ChatResponse, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(&format!("{}/api/chat", base_url))
        .json(&ChatRequest {
            model: model.to_string(),
            messages,
        })
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}