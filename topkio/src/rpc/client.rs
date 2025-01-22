use crate::{
    agent::request::AgentBuilder,
    primitives::{
        constants::{GEMINI_API_URL, OLLAMA_API_URL, OPENAI_API_URL},
        ollama::{ChatCompletion, ChunkResponse, Message},
    },
};
use futures_util::StreamExt;
use serde_json::{from_str, json};
use std::io::Write;

#[non_exhaustive]
#[derive(Clone)]
pub enum Provider {
    OpenAI { api_key: String },
    Gemini { api_key: String },
}
struct ProviderData {
    tag: String,
    api_key: String,
    url: String,
}

impl From<Provider> for ProviderData {
    fn from(provider: Provider) -> Self {
        match provider {
            Provider::OpenAI { api_key } => {
                if api_key == "ollama" {
                    ProviderData {
                        tag: "ollama".to_string(),
                        api_key,
                        url: OLLAMA_API_URL.to_string(),
                    }
                } else {
                    ProviderData {
                        tag: "openai".to_string(),
                        api_key: api_key.clone(),
                        url: OPENAI_API_URL.to_string(),
                    }
                }
            }
            Provider::Gemini { api_key } => ProviderData {
                tag: "gemini".to_string(),
                api_key: api_key.clone(),
                url: GEMINI_API_URL.to_string(),
            },
        }
    }
}

impl Provider {
    fn build_client(&self) -> reqwest::Client {
        let mut headers = reqwest::header::HeaderMap::new();

        match self {
            Provider::OpenAI { api_key } => {
                headers.insert(
                    "Authorization",
                    format!("Bearer {}", api_key)
                        .parse()
                        .expect("Bearer token should parse"),
                );
            }
            Provider::Gemini { api_key } => {
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
            }
        }

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("reqwest client should build")
    }
}

#[derive(Clone)]
pub struct Client {
    pub(crate) url: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    pub fn new(provider: Provider) -> Self {
        let data: ProviderData = provider.clone().into();
        let client = provider.build_client();

        Client {
            url: data.url,
            client,
        }
    }

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

fn parse_chunk(chunk: &str) -> Result<ChunkResponse, serde_json::Error> {
    // 1. Remove "data: " prefix (if present)
    let data_str = chunk.strip_prefix("data: ").unwrap_or(chunk);

    // 2. Find the end of the JSON data (if there are trailing characters)
    let end_index = data_str.find('\n').unwrap_or(data_str.len());
    let json_str = &data_str[..end_index];

    // 3. Deserialize the JSON string
    let result: ChunkResponse = from_str(json_str)?;

    Ok(result)
}
