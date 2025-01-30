use topkio::{AgentBuilder, AtomaClient};

// atoma example
// https://cloud.atoma.network/
#[tokio::main]
async fn main() {
    let client = AtomaClient::new("key");
    let agent = AgentBuilder::new(client, "model_name")
        .stream(true)
        .temperature(0.8)
        .max_tokens(Some(128))
        .build();

    let f = |res: &str| {
        print!("{}", res);
    };

    let prompt = "Entertain me";
    let _ = agent.prompt(prompt, f).await;

    println!("\n");
}
