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
    constants::OPENAI_API_URL,
    primitives::{ChatCompletion, Completion, CompletionRequest, OpenAICompletionRequest},
    utils::parse_chunk,
};
use futures_util::StreamExt;
use std::{cell::OnceCell, io::Write};

pub struct Client {
    pub(crate) url: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        let client = make_client(api_key);

        Client {
            url: OPENAI_API_URL.to_owned(),
            client,
        }
    }

    pub fn with_ollama(url: &str) -> Self {
        let client = make_client("ollama");

        Client {
            url: url.to_owned(),
            client,
        }
    }
}

impl Completion for Client {
    async fn post<F>(&self, req: CompletionRequest, callback: OnceCell<F>) -> Result<(), ()>
    where
        F: Fn(&str) + Send,
    {
        let body = OpenAICompletionRequest::new(&req);
        let url = format!("{}/chat/completions", self.url);
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .expect("openai completion msg");

        match req.stream.unwrap_or(false) {
            true => {
                let mut stream = response.bytes_stream();
                while let Some(item) = stream.next().await {
                    let data = &item.expect("msg");
                    let chunk_str = std::str::from_utf8(data).expect("OpenAI expect utf8.");
                    match parse_chunk(chunk_str) {
                        Ok(response) => {
                            let content = &response.choices[0].delta.content;
                            if let Some(callback) = callback.get() {
                                callback(content);
                                std::io::stdout().flush().expect("Failed to flush stdout");
                            }
                        }
                        Err(err) => println!("OpenAI error parsing chunk: {}", err),
                    }
                }
            }
            false => {
                let chat_completion = response.json::<ChatCompletion>().await.ok();
                if let Some(chat_completion) = chat_completion {
                    chat_completion.choices.iter().for_each(|choice| {
                        let text = &choice.message.content;
                        if let Some(callback) = callback.get() {
                            callback(text);
                        }
                    });
                }
            }
        }

        Ok(())
    }
}

fn make_client(api_key: &str) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", api_key)
            .parse()
            .expect("Bearer token should parse"),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("openai client should build")
}
