# topkio

an innovative LLM framework.  
Minimal footprint, high throughput, ease of use, and streaming capabilities.


## 📦 Installation

```bash
[dependencies]
topkio = "0.1.1"
```

## Quick Start
```rust
use topkio::OpenAIClient;

#[tokio::main]
async fn main() {
    let client = OpenAIClient::with_ollama("http://localhost:11434/v1");

    let agent_builder = client.config("llama3.2");
    let builder = agent_builder.stream(true);
    let builder = builder.temperature(0.8);
    let config = builder.build();

    let prompt = "1 + 1 = ";

    let _ = client
        .prompt(config, prompt, &mut |res| {
            print!("{}", res);
            Ok(())
        })
        .await;

    println!("\n");
}
```