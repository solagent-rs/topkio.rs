#![allow(unused)]

use serde::{Deserialize, Serialize};

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
    pub text: String,
}

impl GeminiResponse {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

#[derive(Debug, Deserialize)]
pub struct Candidate {
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
}

impl GenerateContentRequest {
    pub fn new(prompt: &str) -> Self {
        let part = Part {
            text: prompt.to_string(),
        };
        let content = Content {
            parts: vec![part],
            role: Some("user".to_string()),
        };

        Self {
            contents: vec![content],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerateContentResponse {
    pub(crate) candidates: Vec<ContentCandidate>,
    pub(crate) model_version: String,
    pub(crate) usage_metadata: UsageMetadata,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentCandidate {
    /// Output only. Generated content returned from the model.
    pub content: Content,
    /// Optional. Output only. The reason why the model stopped generating tokens.
    /// If empty, the model has not stopped generating tokens.
    pub finish_reason: Option<FinishReason>,
    /// List of ratings for the safety of a response candidate.
    /// There is at most one rating per category.
    pub safety_ratings: Option<Vec<SafetyRating>>,
    /// Output only. Citation information for model-generated candidate.
    /// This field may be populated with recitation information for any text included in the content.
    /// These are passages that are "recited" from copyrighted material in the foundational LLM's training data.
    pub citation_metadata: Option<CitationMetadata>,
    /// Output only. Token count for this candidate.
    pub token_count: Option<i32>,
    /// Output only.
    pub avg_logprobs: Option<f64>,
    /// Output only. Log-likelihood scores for the response tokens and top tokens
    pub logprobs_result: Option<LogprobsResult>,
    /// Output only. Index of the candidate in the list of response candidates.
    pub index: Option<i32>,
}

/// Gemini Generate Content Response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    /// Default value. This value is unused.
    Unspecified,
    /// Natural stop point of the model or provided stop sequence.
    Stop,
    /// The maximum number of tokens as specified in the request was reached.
    MaxTokens,
    /// The response candidate content was flagged for safety reasons.
    Safety,
    /// The response candidate content was flagged for recitation reasons.
    Recitation,
    /// The response candidate content was flagged for using an unsupported language.
    Language,
    /// Unknown reason.
    Other,
    /// Token generation stopped because the content contains forbidden terms.
    Blocklist,
    /// Token generation stopped for potentially containing prohibited content.
    ProhibitedContent,
    /// Token generation stopped because the content potentially contains Sensitive Personally Identifiable Information (SPII).
    Spii,
    /// The function call generated by the model is invalid.
    MalformedFunctionCall,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    pub citation_sources: Vec<CitationSource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    pub uri: Option<String>,
    pub start_index: Option<i32>,
    pub end_index: Option<i32>,
    pub license: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogProbCandidate {
    pub token: String,
    pub token_id: String,
    pub log_probability: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogprobsResult {
    pub top_candidate: Vec<TopCandidate>,
    pub chosen_candidate: Vec<LogProbCandidate>,
}

#[derive(Debug, Deserialize)]
pub struct TopCandidate {
    pub candidates: Vec<LogProbCandidate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    candidates_token_count: Option<usize>,
    prompt_token_count: Option<usize>,
    total_token_count: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    Unspecified,
    Derogatory,
    Toxicity,
    Violence,
    Sexually,
    Medical,
    Dangerous,
    Harassment,
    HateSpeech,
    SexuallyExplicit,
    DangerousContent,
    CivicIntegrity,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    Unspecified,
    Negligible,
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct CompletionResponse<T> {
    /// The completion choice returned by the completion model provider
    pub choice: ModelChoice,
    /// The raw response returned by the completion model provider
    pub raw_response: T,
}

#[derive(Debug)]
pub enum ModelChoice {
    /// Represents a completion response as a message
    Message(String),
    /// Represents a completion response as a tool call of the form
    /// `ToolCall(function_name, function_params)`.
    ToolCall(String, serde_json::Value),
}

impl TryFrom<GenerateContentResponse> for CompletionResponse<GenerateContentResponse> {
    type Error = String;

    fn try_from(response: GenerateContentResponse) -> Result<Self, Self::Error> {
        match response.candidates.as_slice() {
            [ContentCandidate { content, .. }, ..] => Ok(CompletionResponse {
                choice: match content.parts.first().unwrap() {
                    Part { text } => ModelChoice::Message(text.clone()),
                    Part { .. } => {
                        // let args_value = serde_json::Value::Object(
                        //     function_call.args.clone().unwrap_or_default(),
                        // );
                        // ModelChoice::ToolCall(function_call.name.clone(), args_value)

                        unimplemented!("Tool call not implemented yet");
                    }
                    _ => return Err("Unsupported response by the model of type ".into()),
                },
                raw_response: response,
            }),
            _ => Err("No candidates found in response".into()),
        }
    }
}
