use crate::gemini::data::{GeminiRequest, GeminiResponse, Content, Part, SafetySetting, GenerationConfig};
use topkio_core::models::Message;

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

