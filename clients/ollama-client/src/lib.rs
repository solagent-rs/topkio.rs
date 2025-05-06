use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub message: ChatMessage,
}

pub async fn chat_completion(
    base_url: &str,
    model: &str,
    messages: Vec<ChatMessage>,
) -> Result<ChatResponse, reqwest::Error> {
    println!("Sending request to {} with model {} and messages {:#?}", base_url, model, messages);

    let response = reqwest::Client::new()
    .post(&format!("{}/api/chat", base_url))
    .json(&ChatRequest {
        model: "llama3.2".to_string(),  // Updated model name format
        messages,
        stream: false,
    })
    .send()
    .await.unwrap();
    // .error_for_status()?  // Add proper HTTP error handling
    // .json::<serde_json::Value>()
    // .await?;

    let response = response.json::<ChatResponse>().await?;
    println!("Received response: {:?}", response);

    Ok(response)
}