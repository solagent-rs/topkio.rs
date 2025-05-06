# Topkio - Open Source LLM Gateway

Topkio is an open-source LLM gateway built with Rust and Axum. It provides a unified API for interacting with multiple language models, including OpenAI, Gemini, and Ollama. Topkio simplifies the integration of different LLM providers into your applications, offering a consistent interface and high performance.

## Features

*   **Unified API**: Consistent API calls for various LLM providers (OpenAI, Gemini, Ollama).
*   **High Performance**: Built with Rust for speed and efficiency.
*   **Modular Design**: Easy to extend with new providers and models.
*   **Configuration**: Uses a `topkio.toml` file for easy configuration of providers and models.
*   **Health Checks**: Basic health check functionality for backends.
*   **Model Identifier**: Parses model identifiers to route requests to the correct backend.
*   **Graceful Shutdown**: Implements graceful shutdown using Tokio signals.

## Current Status - Work in Progress (WIP)

Topkio is under active development. Key areas currently being worked on:

*   Implementation of provider APIs (Gemini, Ollama).
*   Dynamic model configuration and management.
*   Comprehensive unit tests and documentation.
*   Improved error handling and validation.

**How to Contribute**

*   **Users**: Report bugs, request features, and provide feedback via [GitHub Issues](https://github.com/zTgx/topkio.rs/issues). Test the OpenAI integration, which is the most stable.
*   **Contributors**: Look for issues labeled `help wanted` or `good first issue`. Focus on adding tests, improving provider implementations, or enhancing documentation.

## Quick Start

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/zTgx/topkio.rs.git
    ```
2.  **Configure `topkio.toml`:**

    Create a `topkio.toml` file in the `config` directory based on the example provided.  Configure your desired LLM providers (Ollama, Gemini, etc.) with their respective API keys and base URLs.

3.  **Run the application:**

    ```bash
    cargo run
    ```

4.  **Send API requests:**

Example `curl` requests:

```bash
curl -X POST http://localhost:3000/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "ollama:llama3.2",
    "messages": [{"role": "user", "content": "Explain AI in 10 words"}],
    "stream": false
  }'
```

```bash
curl -X POST http://localhost:3000/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemini:gemini-2.0-flash",
    "messages": [
      {"role": "user", "content": "Explain AI in 10 words"}
    ],
    "stream": false
  }'
```

    Rust example using `reqwest`:

```rust
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post("http://localhost:3000/chat/completions")
        .header("Content-Type", "application/json")
        .body(json!({
            "model": "ollama:llama3.2",
            "messages": [{"role": "user", "content": "Explain AI in 10 words"}],
            "stream": false
        }).to_string())
        .send()
        .await?;

    println!("Response: {:?}", response);
    Ok(())
}
```

## Configuration

The `topkio.toml` file is used to configure the gateway.  Here's an example:

```toml
[server]
host = "0.0.0.0"
port = 3000

[providers.ollama]
enabled = true
url = "http://localhost:11434"

[providers.gemini]
enabled = true
api_key = "YOUR_GEMINI_API_KEY"
url = "https://generativelanguage.googleapis.com"
```

## License
Apache License 2.0