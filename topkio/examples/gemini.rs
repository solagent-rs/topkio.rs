use topkio::{AgentBuilder, GeminiClient};

#[tokio::main]
async fn main() {
    let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let client = GeminiClient::new(&gemini_api_key);

    let agent = AgentBuilder::new(client, "gemini-1.5-flash")
        .stream(true)
        .temperature(0.8)
        .build();
    // let prompt = "Entertain me";
    let prompt = "1 + 1 = ";

    let f = |res: &str| {
        print!("{}", res);
    };

    let _ = agent.prompt(prompt, f).await;

    println!("\n");
}
