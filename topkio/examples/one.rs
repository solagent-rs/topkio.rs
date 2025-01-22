use topkio::{Client, Provider};

#[tokio::main]
async fn main() {
    let client = Client::new(Provider::OpenAI {
        api_key: "ollama".into(),
    });

    // let openai_api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    // let client = Client::new(Provider::OpenAI {
    //     api_key: openai_api_key,
    // });

    // let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    // let client = Client::new(Provider::Gemini {
    //     api_key: gemini_api_key,
    // });

    let agent_builder = client.agent("llama3.2");
    let builder = agent_builder.stream(true);
    let builder = builder.temperature(0.8);

    let _ = client
        .prompt(builder, "Entertain me", &mut |res| {
            print!("{}", res);
            Ok(())
        })
        .await;
}
