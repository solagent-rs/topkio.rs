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
    agent::AgentBuilder,
    constants::OPENAI_API_URL,
    primitives::{ChatCompletion, Message},
    utils::parse_chunk,
};
use futures_util::StreamExt;
use serde_json::json;
use std::io::Write;

pub struct Client {
    pub(crate) url: String,
    pub(crate) client: reqwest::Client,
}

fn make_build(api_key: &str) -> reqwest::Client {
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

impl Client {
    pub fn new(api_key: &str) -> Self {
        let client = make_build(api_key);

        Client {
            url: OPENAI_API_URL.to_owned(),
            client,
        }
    }

    pub fn with_ollama(url: &str) -> Self {
        let client = make_build("ollama");

        Client {
            url: url.to_owned(),
            client,
        }
    }
}

impl Client {
    pub async fn prompt<F>(&self, builder: AgentBuilder, prompt: &str, callback: &mut F)
    where
        F: Fn(&String) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        let agent = builder.build();
        println!(">> agent: {:?}", agent);

        let full_history = vec![Message {
            role: "user".into(),
            content: prompt.into(),
        }];

        let req = json!({
            "model": agent.model,
            "messages": full_history,
            "temperature": agent.temperature,
            "stream": agent.stream,
        });
        let url = format!("{}/chat/completions", self.url);
        println!("url: {}", url);
        println!("req: {}", serde_json::to_string(&req).unwrap());

        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .expect("stream msg");

        let is_stream = agent.stream.unwrap_or(false);
        if is_stream {
            let mut stream = response.bytes_stream();
            while let Some(item) = stream.next().await {
                // println!("Chunk: {:?}", item?);

                let data = &item.expect("msg");
                let chunk_str = std::str::from_utf8(data).unwrap();
                match parse_chunk(chunk_str) {
                    Ok(response) => {
                        // chunks.push(response);
                        let content = &response.choices[0].delta.content;
                        let _ = callback(&content);

                        std::io::stdout().flush().expect("Failed to flush stdout");
                    }
                    Err(err) => println!("Error parsing chunk: {}", err),
                }
            }
        } else {
            let result: Result<ChatCompletion, serde_json::Error> =
                serde_json::from_value(response.json().await.unwrap());

            match result {
                Ok(chat_completion) => {
                    println!("解析成功: {:?}", chat_completion);
                    // 访问具体字段，例如：
                    // println!("生成的文本: {}", chat_completion.choices[0].message.content);
                    let _ = callback(&chat_completion.choices[0].message.content);
                }
                Err(err) => {
                    eprintln!("解析失败: {}", err);
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
