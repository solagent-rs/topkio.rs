use {
    crate::gemini::primitive::{
        CompletionResponse, GeminiResponse, GenerateContentRequest, GenerateContentResponse,
        ModelChoice,
    },
    topkio_primitive::api::Message,
};

pub async fn chat_completion(
    base_url: &str,
    api_key: &str,
    model: &str,
    messages: Vec<Message>,
    stream: Option<bool>,
) -> Result<GeminiResponse, anyhow::Error> {
    println!("Gemini chat completion request: {:?}", messages);

    let mut text = String::new();

    let eable_stream = stream.unwrap_or(false);
    let endpoint = match eable_stream {
        true => format!(
            "{}/{}:{}?key={}",
            base_url, model, "streamGenerateContent", api_key,
        ),
        false => format!(
            "{}/{}:{}?key={}",
            base_url, model, "generateContent", api_key,
        ),
    };

    println!("Gemini endpoint: {}", endpoint);

    let body = GenerateContentRequest::new(messages[0].content.as_str());
    let response = reqwest::Client::new()
        .post(&endpoint)
        .json(&body)
        .send()
        .await
        .expect("gemini generate content msg");

    match eable_stream {
        true => {
            unimplemented!("Streaming not implemented yet");

            // let mut stream = response.bytes_stream();
            // while let Some(item) = stream.next().await {
            //     let data = &item.expect("msg");
            //     let chunk_str = std::str::from_utf8(data).expect("Gemini expect utf8.");
            //     if let Ok(generate_response) = gemini_parse_chunk(chunk_str) {
            //         if let Ok(completion_response) =
            //             CompletionResponse::try_from(generate_response)
            //         {
            //             match completion_response {
            //                 CompletionResponse {
            //                     choice: ModelChoice::Message(msg),
            //                     ..
            //                 } => {
            //                     if let Some(callback) = callback.get() {
            //                         callback(&msg);
            //                         std::io::stdout().flush().expect("Failed to flush stdout");
            //                     }
            //                 }
            //                 CompletionResponse {
            //                     choice: ModelChoice::ToolCall(toolname, args),
            //                     ..
            //                 } => {
            //                     if let Ok(res) = tools.invoke(&toolname, args.to_string()).await
            //                     {
            //                         if let Some(callback) = callback.get() {
            //                             callback(&res);
            //                         }
            //                     }
            //                 }
            //             }
            //         }
            //     };
            // }
        }
        false => {
            let generate_response = response.json::<GenerateContentResponse>().await;
            match generate_response {
                Ok(generate_response) => {
                    if let Ok(completion_response) = CompletionResponse::try_from(generate_response)
                    {
                        match completion_response {
                            CompletionResponse {
                                choice: ModelChoice::Message(msg),
                                ..
                            } => {
                                // if let Some(callback) = callback.get() {
                                //     callback(&msg);
                                // }

                                println!("Gemini response: {}", msg);

                                text = msg;
                            }
                            CompletionResponse {
                                choice: ModelChoice::ToolCall(toolname, args),
                                ..
                            } => {
                                // if let Ok(res) = tools.invoke(&toolname, args.to_string()).await
                                // {
                                //     if let Some(callback) = callback.get() {
                                //         callback(&res);
                                //     }
                                // }

                                println!("Gemini tool call: {} with args: {}", toolname, args);
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

    Ok(GeminiResponse::new(text))
}
