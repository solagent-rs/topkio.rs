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

use crate::{
    constants::{GEMINI_API_URL, GEMINI_API_URL_PATH},
    primitives::{Completion, CompletionRequest, GenerateContentRequest, GenerateContentResponse},
    utils::gemini_parse_chunk,
};
use futures_util::StreamExt;
use std::{cell::OnceCell, io::Write};

pub struct Client {
    pub(crate) api_key: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("openai client should build");

        Client {
            api_key: api_key.to_owned(),
            client,
        }
    }
}

impl Completion for Client {
    async fn post<F>(&self, req: CompletionRequest, callback: OnceCell<F>) -> Result<(), ()>
    where
        F: Fn(&str) + std::marker::Send,
    {
        let eable_stream = req.stream.unwrap_or(false);
        let endpoint = match eable_stream {
            true => format!(
                "{}/{}/{}:{}?key={}",
                GEMINI_API_URL,
                GEMINI_API_URL_PATH,
                req.model,
                "streamGenerateContent",
                self.api_key,
            ),
            false => format!(
                "{}/{}/{}:{}?key={}",
                GEMINI_API_URL, GEMINI_API_URL_PATH, req.model, "generateContent", self.api_key,
            ),
        };

        let body = GenerateContentRequest::new(&req.prompt, None);
        let response = self
            .client
            .post(&endpoint)
            .json(&body)
            .send()
            .await
            .expect("gemini generate content msg");

        match eable_stream {
            true => {
                let mut stream = response.bytes_stream();
                while let Some(item) = stream.next().await {
                    let data = &item.expect("msg");
                    let chunk_str = std::str::from_utf8(data).expect("Gemini expect utf8.");
                    if let Ok(response) = gemini_parse_chunk(chunk_str) {
                        response.candidates.iter().for_each(|candidate| {
                            candidate.content.parts.iter().for_each(|part| {
                                if let Some(callback) = callback.get() {
                                    callback(&part.text);
                                    std::io::stdout().flush().expect("Failed to flush stdout");
                                }
                            })
                        });
                    };
                }
            }
            false => {
                let generate_response = response.json::<GenerateContentResponse>().await.ok();
                if let Some(generate_response) = generate_response {
                    generate_response.candidates.iter().for_each(|candidate| {
                        candidate.content.parts.iter().for_each(|part| {
                            if let Some(callback) = callback.get() {
                                callback(&part.text);
                            }
                        });
                    })
                }
            }
        }

        Ok(())
    }
}
