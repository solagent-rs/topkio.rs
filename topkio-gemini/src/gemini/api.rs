use {
    super::chat_completion::chat_completion,
    topkio_core::{
        backend::Backend,
        models::{ChatCompletionResponse, Message},
    },
};

pub struct GeminiBackend {
    base_url: String,
    api_key: String,
}

impl GeminiBackend {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self { base_url, api_key }
    }
}

#[async_trait::async_trait]
impl Backend for GeminiBackend {
    async fn chat_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        stream: Option<bool>,
    ) -> Result<ChatCompletionResponse, anyhow::Error> {
        let response =
            chat_completion(&self.base_url, &self.api_key, model, messages, stream).await?;

        Ok(ChatCompletionResponse {
            message: Message {
                role: "assistant".to_string(),
                content: response.text,
            },
        })
    }

    async fn health_check(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
