

mod config;
mod error;
use error::ApiError;
mod shutdown;
mod handlers;

use axum::{Router, routing::post};
use std::{sync::Arc, collections::HashMap};
use crate::config::GatewayConfig;
use handlers::handle_chat_completion;
use topkio_core::backend::Backend;
use topkio_ollama::OllamaBackend;
use topkio_gemini::GeminiBackend;
use shutdown::shutdown_signal;
use crate::shutdown::ShutdownConfig;

struct AppState {
    backends: HashMap<String, Arc<dyn Backend>>,
    config: GatewayConfig,
}

async fn initialize_backends(
    config: &GatewayConfig,
) -> anyhow::Result<HashMap<String, Arc<dyn Backend>>> {
    let mut backends: HashMap<String, Arc<dyn Backend>> = HashMap::new();

    // Ollama (optional)
    if let Some(ollama_cfg) = &config.providers.ollama {
        let ollama_backend = OllamaBackend::new(
            ollama_cfg.url.clone(),
        );
        ollama_backend.health_check().await?;
        backends.insert("ollama".to_string(), Arc::new(ollama_backend));
    }

    println!("Ollama backend initialized");
    
    // Gemini (optional)
    if let Some(gemini_cfg) = &config.providers.gemini {
        let gemini_backend = GeminiBackend::new(
            gemini_cfg.url.clone(),
            gemini_cfg.api_key.clone().unwrap(),
        );
        // gemini_backend.health_check().await?;
        backends.insert("gemini".to_string(), Arc::new(gemini_backend));
    }
    
    println!("Backends initialized: {:?}", backends.keys());

    Ok(backends)
}

pub async fn start() -> anyhow::Result<()> {
    println!("Starting Topkio Gateway...");

    let config = GatewayConfig::load("config/topkio.toml")?;
    let backends = initialize_backends(&config).await?;
    
    let state = Arc::new(AppState { backends, config });

    let app = Router::new()
        .route("/chat/completions", post(handle_chat_completion))
        .with_state(state.clone());

    // Get the server address from config
    let addr = format!("{}:{}", &state.config.server.host, &state.config.server.port)
    .parse::<String>()
    .expect("Invalid server address configuration");

    println!("Starting server on http://{}", addr);

    // Create TCP listener with configurable options
    let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .unwrap_or_else(|_| panic!("Failed to bind to address {}", addr));

    println!("Server running on http://{} (Press CTRL+C to stop)", addr);
    let shutdown_config = ShutdownConfig {
        graceful_timeout: tokio::time::Duration::from_secs(state.config.server.graceful_shutdown_seconds),
        enable_ctrl_c: true,
        enable_signal: false,
        enable_custom: state.config.server.enable_custom_shutdown,
    };
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal(shutdown_config)).await?;

    Ok(())
}