// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};
use std::{cell::OnceCell, collections::HashMap};

/////////////////////////////////////////////// Completion trait //////////////////////////////////
pub trait Completion {
    fn post<F>(
        &self,
        req: CompletionRequest,
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
    pub parameters: ToolParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tools {
    pub function_declarations: Vec<FunctionDeclaration>,
}

/////////////////////////////////////////////// Request & Response ///////////////////////////////////////////
/// Gemini
/// Generate Content Request
#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub text: String,
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
    pub fn new(prompt: &str, tools: Option<Tools>) -> Self {
        let part = Part {
            text: prompt.to_string(),
        };
        let content = Content {
            parts: vec![part],
            role: "user".to_string(),
        };

        Self {
            contents: vec![content],
            tools,
        }
    }
}

/// Gemini Generate Content Response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Candidate {
    pub(crate) content: Content,
    pub(crate) finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UsageMetadata {
    prompt_token_count: usize,
    candidates_token_count: Option<usize>,
    total_token_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerateContentResponse {
    pub(crate) candidates: Vec<Candidate>,
    pub(crate) usage_metadata: UsageMetadata,
    pub(crate) model_version: String,
}

/// OpenAI
/// Completion Request
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
    pub temperature: Option<f64>,
}

impl OpenAICompletionRequest {
    pub fn new(req: &CompletionRequest) -> Self {
        let full_history = vec![Message {
            role: "user".into(),
            content: req.prompt.clone(),
        }];

        OpenAICompletionRequest {
            model: req.model.clone(),
            messages: full_history,
            stream: req.stream,
            temperature: req.temperature,
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
pub struct ChoiceFalse {
    pub index: usize,
    pub message: Message,
    pub finish_reason: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Delta {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: usize,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ChunkResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<Choice>,
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

    pub fn tools(mut self, tools: Option<Vec<FunctionDeclaration>>) -> Self {
        self.req.tools = tools;
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
