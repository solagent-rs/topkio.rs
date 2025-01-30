use serde::{Deserialize, Serialize};
use std::{cell::OnceCell, collections::HashMap};

use crate::ToolSet;

/////////////////////////////////////////////// Completion trait //////////////////////////////////
pub trait Completion {
    fn post<F>(
        &self,
        req: CompletionRequest,
        tools: &ToolSet,
        callback: OnceCell<F>,
    ) -> impl std::future::Future<Output = Result<(), ()>> + Send
    where
        F: Fn(&str) + Send + 'static;
}

/////////////////////////////////////////////// Tools ///////////////////////////////////////////
/// For all kinds of client: openai, gemini and more
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolParameter {
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolParameters {
    pub properties: HashMap<String, ToolParameter>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDeclaration {
    pub description: String,
    pub name: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tools {
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
}

impl Tools {
    pub fn new(function_declarations: Option<Vec<FunctionDeclaration>>) -> Self {
        Tools {
            function_declarations,
        }
    }
}

/////////////////////////////////////////////// Request & Response ///////////////////////////////////////////
/// Gemini
/// Generate Content Request
///
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    /// The IANA standard MIME type of the source data. Examples: - image/png - image/jpeg
    /// If an unsupported MIME type is provided, an error will be returned.
    pub mime_type: String,
    /// Raw bytes for media formats. A base64-encoded string.
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionCall {
    /// Required. The name of the function to call. Must be a-z, A-Z, 0-9, or contain underscores
    /// and dashes, with a maximum length of 63.
    pub name: String,
    /// Optional. The function parameters and values in JSON object format.
    pub args: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionResponse {
    /// The name of the function to call. Must be a-z, A-Z, 0-9, or contain underscores and dashes,
    /// with a maximum length of 63.
    pub name: String,
    /// The function response in JSON object format.
    pub response: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileData {
    /// Optional. The IANA standard MIME type of the source data.
    pub mime_type: Option<String>,
    /// Required. URI.
    pub file_uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExecutableCode {
    /// Programming language of the code.
    pub language: ExecutionLanguage,
    /// The code to be executed.
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionLanguage {
    /// Unspecified language. This value should not be used.
    LanguageUnspecified,
    /// Python >= 3.10, with numpy and simpy available.
    Python,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeExecutionResult {
    /// Outcome of the code execution.
    pub outcome: CodeExecutionOutcome,
    /// Contains stdout when code execution is successful, stderr or other description otherwise.
    pub output: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CodeExecutionOutcome {
    /// Unspecified status. This value should not be used.
    Unspecified,
    /// Code execution completed successfully.
    Ok,
    /// Code execution finished but with a failure. stderr should contain the reason.
    Failed,
    /// Code execution ran for too long, and was cancelled. There may or may not be a partial output present.
    DeadlineExceeded,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<Blob>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response: Option<FunctionResponse>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<FileData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_code: Option<ExecutableCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_execution_result: Option<CodeExecutionResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Tools>,
}

impl GenerateContentRequest {
    pub fn new(prompt: &str, function_declarations: Option<Vec<FunctionDeclaration>>) -> Self {
        let part = Part {
            text: Some(prompt.to_string()),
            ..Default::default()
        };
        let content = Content {
            parts: vec![part],
            role: "user".to_string(),
        };

        Self {
            contents: vec![content],
            tools: Some(Tools::new(function_declarations)),
        }
    }
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerateContentResponse {
    pub(crate) candidates: Vec<ContentCandidate>,
    pub(crate) model_version: String,
    pub(crate) usage_metadata: UsageMetadata,
}

#[derive(Debug)]
pub enum ModelChoice {
    /// Represents a completion response as a message
    Message(String),
    /// Represents a completion response as a tool call of the form
    /// `ToolCall(function_name, function_params)`.
    ToolCall(String, serde_json::Value),
}

#[derive(Debug)]
pub struct CompletionResponse<T> {
    /// The completion choice returned by the completion model provider
    pub choice: ModelChoice,
    /// The raw response returned by the completion model provider
    pub raw_response: T,
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

impl TryFrom<GenerateContentResponse> for CompletionResponse<GenerateContentResponse> {
    type Error = String;

    fn try_from(response: GenerateContentResponse) -> Result<Self, Self::Error> {
        match response.candidates.as_slice() {
            [ContentCandidate { content, .. }, ..] => Ok(CompletionResponse {
                choice: match content.parts.first().unwrap() {
                    Part {
                        text: Some(text), ..
                    } => ModelChoice::Message(text.clone()),
                    Part {
                        function_call: Some(function_call),
                        ..
                    } => {
                        let args_value = serde_json::Value::Object(
                            function_call.args.clone().unwrap_or_default(),
                        );
                        ModelChoice::ToolCall(function_call.name.clone(), args_value)
                    }
                    _ => return Err("Unsupported response by the model of type ".into()),
                },
                raw_response: response,
            }),
            _ => Err("No candidates found in response".into()),
        }
    }
}

/// OpenAI
/// Completion Request

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: Function,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// "system", "user", or "assistant"
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(role: &str, content: &str) -> Self {
        Message {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAICompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
    pub temperature: Option<f64>,
    pub tools: Option<Vec<FunctionDeclaration>>,
}

impl OpenAICompletionRequest {
    pub fn new(req: CompletionRequest) -> Self {
        let full_history = vec![Message {
            role: "user".into(),
            content: req.prompt.clone(),
        }];

        OpenAICompletionRequest {
            model: req.model.clone(),
            messages: full_history,
            stream: req.stream,
            temperature: req.temperature,
            tools: Some(req.tools.unwrap_or_default()),
        }
    }
}

/// OpenAI
/// Completion Response
#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceFalse {
    pub index: usize,
    pub message: OpenAIMessage,
    pub finish_reason: String,
    pub logprobs: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<ChoiceFalse>,
    pub usage: Usage,
}

impl TryFrom<ChatCompletion> for CompletionResponse<ChatCompletion> {
    type Error = String;

    fn try_from(value: ChatCompletion) -> std::prelude::v1::Result<Self, Self::Error> {
        match value.choices.as_slice() {
            [ChoiceFalse {
                message:
                    OpenAIMessage {
                        tool_calls: Some(calls),
                        ..
                    },
                ..
            }, ..] => {
                let call = calls.first().unwrap();

                Ok(CompletionResponse {
                    choice: ModelChoice::ToolCall(
                        call.function.name.clone(),
                        serde_json::from_str(&call.function.arguments).unwrap_or_default(),
                    ),
                    raw_response: value,
                })
            }
            [ChoiceFalse {
                message:
                    OpenAIMessage {
                        content: Some(content),
                        ..
                    },
                ..
            }, ..] => Ok(CompletionResponse {
                choice: ModelChoice::Message(content.to_string()),
                raw_response: value,
            }),
            _ => Err("Response did not contain a message or tool call".into()),
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Delta {
//     pub role: String,
//     pub content: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: usize,
    pub delta: OpenAIMessage,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<Choice>,
}

impl TryFrom<ChunkResponse> for CompletionResponse<ChunkResponse> {
    type Error = String;

    fn try_from(value: ChunkResponse) -> std::prelude::v1::Result<Self, Self::Error> {
        match value.choices.as_slice() {
            [Choice {
                delta:
                    OpenAIMessage {
                        tool_calls: Some(calls),
                        ..
                    },
                ..
            }, ..] => {
                let call = calls.first().unwrap();

                Ok(CompletionResponse {
                    choice: ModelChoice::ToolCall(
                        call.function.name.clone(),
                        serde_json::from_str(&call.function.arguments).unwrap_or_default(),
                    ),
                    raw_response: value,
                })
            }
            [Choice {
                delta:
                    OpenAIMessage {
                        content: Some(content),
                        ..
                    },
                ..
            }, ..] => Ok(CompletionResponse {
                choice: ModelChoice::Message(content.to_string()),
                raw_response: value,
            }),
            _ => Err("Response did not contain a message or tool call".into()),
        }
    }
}

/// CompletionRequest

#[derive(Default)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub stream: Option<bool>,
    pub preamble: Option<String>,
    pub chat_history: Vec<Message>,
    pub tools: Option<Vec<FunctionDeclaration>>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
    pub additional_params: Option<serde_json::Value>,
}

pub struct CompletionRequestBuilder {
    pub req: CompletionRequest,
}

impl CompletionRequestBuilder {
    pub fn new() -> Self {
        CompletionRequestBuilder {
            req: CompletionRequest::default(),
        }
    }

    pub fn model(mut self, model: String) -> Self {
        self.req.model = model;
        self
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.req.prompt = prompt;
        self
    }

    pub fn stream(mut self, stream: Option<bool>) -> Self {
        self.req.stream = stream;
        self
    }

    pub fn preamble(mut self, preamble: Option<String>) -> Self {
        self.req.preamble = preamble;
        self
    }

    pub fn temperature(mut self, temparature: Option<f64>) -> Self {
        self.req.temperature = temparature;
        self
    }

    pub fn chat_history(mut self, chat_history: Vec<Message>) -> Self {
        self.req.chat_history = chat_history;
        self
    }

    pub fn tools(mut self, tools: Vec<FunctionDeclaration>) -> Self {
        self.req.tools = Some(tools);
        self
    }

    pub fn max_tokens(mut self, max_tokens: Option<u64>) -> Self {
        self.req.max_tokens = max_tokens;
        self
    }

    pub fn additional_params(mut self, additional_params: Option<serde_json::Value>) -> Self {
        self.req.additional_params = additional_params;
        self
    }

    pub fn build(self) -> CompletionRequest {
        self.req
    }
}
