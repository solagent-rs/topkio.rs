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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// "system", "user", or "assistant"
    pub role: String,
    pub content: String,
}

///////////////// false
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
#[serde(rename_all = "camelCase")]
pub struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

/// gemini
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
pub struct Structure {
    pub contents: Vec<Content>,
}

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
