use crate::{
    constants::{GEMINI_API_URL, GEMINI_API_URL_PATH},
    primitives::{
        Completion, CompletionRequest, CompletionResponse, GenerateContentRequest,
        GenerateContentResponse, ModelChoice,
    },
    utils::gemini_parse_chunk,
    ToolSet,
};
use futures_util::StreamExt;
use std::{cell::OnceCell, io::Write};

pub struct Client<'a> {
    pub(crate) api_key: String,
    pub(crate) client: reqwest::Client,
    pub tools: Option<&'a ToolSet>,
}

impl<'a> Client<'a> {
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
            tools: None,
        }
    }

    pub fn tools(&mut self, toolset: &'a ToolSet) {
        self.tools = Some(toolset);
    }
}

impl Completion for Client<'_> {
    async fn post<F>(
        &self,
        req: CompletionRequest,
        tools: &ToolSet,
        callback: OnceCell<F>,
    ) -> Result<(), ()>
    where
        F: Fn(&str) + Send,
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

        let body = GenerateContentRequest::new(&req.prompt, req.tools);
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
                    if let Ok(generate_response) = gemini_parse_chunk(chunk_str) {
                        if let Ok(completion_response) =
                            CompletionResponse::try_from(generate_response)
                        {
                            match completion_response {
                                CompletionResponse {
                                    choice: ModelChoice::Message(msg),
                                    ..
                                } => {
                                    if let Some(callback) = callback.get() {
                                        callback(&msg);
                                        std::io::stdout().flush().expect("Failed to flush stdout");
                                    }
                                }
                                CompletionResponse {
                                    choice: ModelChoice::ToolCall(toolname, args),
                                    ..
                                } => {
                                    if let Ok(res) = tools.invoke(&toolname, args.to_string()).await
                                    {
                                        if let Some(callback) = callback.get() {
                                            callback(&res);
                                        }
                                    }
                                }
                            }
                        }
                    };
                }
            }
            false => {
                let generate_response = response.json::<GenerateContentResponse>().await;
                match generate_response {
                    Ok(generate_response) => {
                        if let Ok(completion_response) =
                            CompletionResponse::try_from(generate_response)
                        {
                            match completion_response {
                                CompletionResponse {
                                    choice: ModelChoice::Message(msg),
                                    ..
                                } => {
                                    if let Some(callback) = callback.get() {
                                        callback(&msg);
                                    }
                                }
                                CompletionResponse {
                                    choice: ModelChoice::ToolCall(toolname, args),
                                    ..
                                } => {
                                    if let Ok(res) = tools.invoke(&toolname, args.to_string()).await
                                    {
                                        if let Some(callback) = callback.get() {
                                            callback(&res);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Gemini parse GenerateContentResponse error: {}", e);
                    }
                }
            }
        }

        Ok(())
    }
}
