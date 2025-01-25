# topkio

Not all AI agent frameworks are called topkio.  

Small, fast, powerful, and easy to use.  

WIP

## Features
- [x] OpenAI
- [x] Ollama
- [x] Gemini
- [x] streaming
- [x] tool
- [ ] embeddings

## Quick Start
```rust
use topkio::{AgentBuilder, OpenAIClient};

#[tokio::main]
async fn main() {
    let client = OpenAIClient::with_ollama("http://localhost:11434/v1");
    let agent = AgentBuilder::new(client, "llama3.2")
        .stream(true)
        .temperature(0.8)
        .build();

    let f = |res: &str| {
        print!("{}", res);
    };

    let prompt = "Entertain me";
    let _ = agent.prompt(prompt, f).await;

    println!("\n");
}
```