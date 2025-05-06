use serde::{Deserialize, Serialize};
use topkio_core::models::Message;

#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct SafetySetting {
    pub category: String,
    pub threshold: String,
}

#[derive(Debug, Serialize)]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
pub struct Candidate {
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

pub async fn chat_completion(
    base_url: &str,
    api_key: &str,
    model: &str,
    messages: Vec<Message>,
) -> Result<GeminiResponse, anyhow::Error> {
    let url = format!("{}/{}:generateContent?key={}", base_url, model, api_key);
    
    let contents = messages.into_iter().map(|m| Content {
        parts: vec![Part { text: m.content }],
        role: Some(m.role),
    }).collect();

    let request = GeminiRequest {
        contents,
        safety_settings: Some(vec![
            SafetySetting {
                category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
                threshold: "BLOCK_ONLY_HIGH".to_string(),
            }
        ]),
        generation_config: Some(GenerationConfig {
            temperature: Some(0.9),
            top_p: Some(0.9),
            max_output_tokens: Some(1024),
        }),
    };

    let response = reqwest::Client::new()
        .post(&url)
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json::<GeminiResponse>()
        .await?;

    Ok(response)
}