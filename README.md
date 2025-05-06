# Topkio

Topkio is an open-source LLM aggregation platform, built with Rust and Axum, providing a unified API interface for multiple models including OpenAI, Gemini, Ollama, and more.

## Features
- Unified API calls for LLM providers such as OpenAI, Gemini, Ollama, and DeepSeek.
- High-performance API service powered by Rust.
- Modular design, making it easy to extend with new providers.

## Work in Progress (WIP)
Topkio is actively under development, and some features are still being refined. Current WIP areas include:
- Full implementation of Gemini, Ollama, and DeepSeek provider APIs.
- Dynamic model configuration and streaming response support.
- Comprehensive unit tests and documentation.

**Tips for WIP**:
- **For Users**: Check the [Issues](https://github.com/zTgx/topkio.rs/issues) page for known bugs or limitations. Test with OpenAI, as it has the most stable implementation.
- **For Contributors**: Focus on open issues labeled `help wanted` or `good first issue`. Start with small tasks like adding tests or improving provider implementations.
- **Feedback**: Share your experience or suggestions via GitHub Issues to help shape the project’s direction.

## Quick Start
1. Clone the repository: `git clone https://github.com/zTgx/topkio.rs.git`
2. Configure `topkio.toml` (see example).
3. Run: `cargo run`

```shell
curl -X POST http://localhost:8080/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "ollama::llama3.2", 
    "messages": [{"role": "user", "content": "Explain AI in 10 words"}]
  }'

curl -X POST http://localhost:3000/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemini:gemini-pro",
    "messages": [
      {"role": "user", "content": "Explain quantum computing in simple terms"}
    ]
  }'
```

## License
Apache License 2.0


