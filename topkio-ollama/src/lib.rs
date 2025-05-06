use topkio_core::backend::Backend;
use topkio_core::models::{ChatCompletionRequest, ChatCompletionResponse, Message};

pub async fn chat_completion(
    base_url: &str,
    model: &str,
    messages: Vec<Message>,
    stream: Option<bool>,
) -> Result<ChatCompletionResponse, reqwest::Error> {
    println!(
        "Sending request to {} with model {} and messages {:#?}",
        base_url, model, messages
    );

    let response = reqwest::Client::new()
        .post(&format!("{}/api/chat", base_url))
        .json(&ChatCompletionRequest {
            model: "llama3.2".to_string(), // Updated model name format
            messages,
            stream: stream,
        })
        .send()
        .await
        .unwrap();
    // .error_for_status()?  // Add proper HTTP error handling
    // .json::<serde_json::Value>()
    // .await?;

    let response = response.json::<ChatCompletionResponse>().await?;
    println!("Received response: {:?}", response);

    Ok(response)
}

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
