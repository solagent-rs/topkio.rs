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

use futures_util::StreamExt;
use regex::Regex;
use std::io::Write;

use crate::{
    agent::AgentBuilder, constants::GEMINI_API_URL, primitives::GenerateContentResponse,
    utils::build_structure,
};

pub struct Client {
    pub(crate) api_key: String,
    pub(crate) client: reqwest::Client,
}

fn make_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Gemini client should build")
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Client {
            api_key: api_key.to_owned(),
            client: make_client(),
        }
    }
}

impl Client {
    pub async fn prompt<F>(&self, builder: AgentBuilder, prompt: &str, callback: &mut F)
    where
        F: Fn(&String) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        let agent = builder.build();
        let req = build_structure(prompt);

        let stream = agent.stream.unwrap_or(false);
        let url = if stream {
            format!(
                "{}/{}?key={}",
                GEMINI_API_URL,
                "v1beta/models/gemini-1.5-flash:streamGenerateContent",
                self.api_key
            )
        } else {
            format!(
                "{}/{}?key={}",
                GEMINI_API_URL, "v1beta/models/gemini-1.5-flash:generateContent", self.api_key
            )
        };

        println!("url: {}", url);
        println!("req: {}", serde_json::to_string(&req).unwrap());

        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .expect("stream msg");

        if stream {
            let text_regex = Regex::new(r#"text": (.*?)\n"#).unwrap();

            let mut stream = response.bytes_stream();
            while let Some(item) = stream.next().await {
                let data = &item.expect("msg");
                let chunk_str = std::str::from_utf8(data).unwrap();
                if let Some(text) = text_regex.captures(chunk_str) {
                    let content = text.get(1).map_or("", |m| m.as_str());
                    let _ = callback(&content.to_owned());

                    std::io::stdout().flush().expect("Failed to flush stdout");
                }
            }
        } else {
            let result: Result<GenerateContentResponse, serde_json::Error> =
                serde_json::from_value(response.json().await.unwrap());

            match result {
                Ok(chat_completion) => {
                    let content = &chat_completion.candidates[0].content.parts[0].text;
                    let _ = callback(content);
                }
                Err(err) => {
                    eprintln!("Parse error: {}", err);
                }
            }
        }
    }
}

impl Client {
    pub fn agent(&self, model: &str) -> AgentBuilder {
        AgentBuilder::new(model.into())
    }
}
