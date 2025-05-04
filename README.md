<div align="center">

# topkio.rs 
  [<img alt="crates.io" src="https://img.shields.io/crates/v/topkio?style=for-the-badge&logo=rust">](https://crates.io/crates/topkio)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/topkio?style=for-the-badge&logo=docs.rs">](https://docs.rs/topkio)
  [<img alt="crates.io" src="https://img.shields.io/crates/d/topkio?style=for-the-badge&logo=rust">](https://crates.io/crates/topkio)
</div>

Simple and easy to use LLM Router in Rust.

</br>

```shell
curl -X POST http://localhost:8080/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4", "messages": [{"role": "user", "content": "Hello, world!"}]}'
```

topkio/
├── topkio-core/                # 核心逻辑（配置、模型、通用客户端）
│   ├── src/
│   │   ├── lib.rs
│   │   ├── config.rs
│   │   ├── client.rs
│   │   ├── error.rs
│   │   └── models.rs
│   └── Cargo.toml
├── topkio-providers/           # LLM 提供商实现
│   ├── src/
│   │   ├── lib.rs
│   │   ├── provider.rs      # Provider trait 和路由逻辑
│   │   ├── openai.rs       # OpenAI 提供商实现
│   │   ├── gemini.rs       # Gemini 提供商实现
│   │   ├── ollama.rs       # Ollama 提供商实现
│   │   └── deepseek.rs     # DeepSeek 提供商实现
│   └── Cargo.toml
├── topkio-server/              # HTTP 服务器
│   ├── src/
│   │   ├── main.rs
│   │   ├── handler.rs
│   │   └── server.rs
│   └── Cargo.toml
├── topkio.toml              # 配置文件
├── Cargo.toml               # Workspace 根配置文件
└── README.md