<div align="center">

# topkio.rs 
  [<img alt="crates.io" src="https://img.shields.io/crates/v/topkio?style=for-the-badge&logo=rust">](https://crates.io/crates/topkio)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/topkio?style=for-the-badge&logo=docs.rs">](https://docs.rs/topkio)
  [<img alt="crates.io" src="https://img.shields.io/crates/d/topkio?style=for-the-badge&logo=rust">](https://crates.io/crates/topkio)
</div>

</br>

Not all AI agent frameworks are called topkio.  

Small, fast, powerful, and easy to use.  

WIP

## Features
- [x] OpenAI
- [x] Ollama
- [x] Gemini
- [x] Atoma
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
