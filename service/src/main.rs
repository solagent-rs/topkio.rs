mod config;
mod error;
use error::ApiError;
mod handlers;
mod middleware;
mod shutdown;

use {
    crate::{
        config::GatewayConfig,
        shutdown::{shutdown_signal, ShutdownConfig},
    },
    anyhow::Result,
    axum::{routing::post, Router},
    handlers::handle_chat_completion,
    std::{collections::HashMap, sync::Arc},
    topkio_core::api::UnifiedLlmApi,
    topkio_google::GeminiBackend,
    topkio_ollama::OllamaBackend,
};

struct AppState {
    backends: HashMap<String, Arc<dyn UnifiedLlmApi>>,
    config: GatewayConfig,
}

async fn initialize_backends(
    config: &GatewayConfig,
) -> anyhow::Result<HashMap<String, Arc<dyn UnifiedLlmApi>>> {
    let mut backends: HashMap<String, Arc<dyn UnifiedLlmApi>> = HashMap::new();

    // Ollama (optional)
    if let Some(ollama_cfg) = &config.providers.ollama {
        let ollama_backend = OllamaBackend::new(ollama_cfg.url.clone());
        ollama_backend.health_check().await?;
        backends.insert("ollama".to_string(), Arc::new(ollama_backend));
    }

    println!("Ollama backend initialized");

    // Gemini (optional)
    if let Some(gemini_cfg) = &config.providers.gemini {
        let gemini_backend =
            GeminiBackend::new(gemini_cfg.url.clone(), gemini_cfg.api_key.clone().unwrap());
        // gemini_backend.health_check().await?;
        backends.insert("gemini".to_string(), Arc::new(gemini_backend));
    }

    println!("Backends initialized: {:?}", backends.keys());

    Ok(backends)
}

pub async fn start() -> Result<()> {
    println!("Starting Topkio Gateway...");

    let config = GatewayConfig::load("config/topkio.toml")?;
    let backends = initialize_backends(&config).await?;

    let app_state = Arc::new(AppState { backends, config });

    let app = Router::new()
        .route("/chat/completions", post(handle_chat_completion))
        // .layer(axum::middleware::from_fn_with_state(app_state.clone(), crate::middleware::auth_middleware))
        .with_state(app_state.clone());

    // Get the server address from config
    let addr = format!(
        "{}:{}",
        &app_state.config.server.host, &app_state.config.server.port
    )
    .parse::<String>()
    .expect("Invalid server address configuration");

    // Create TCP listener with configurable options
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", addr));

    println!("Server running on http://{} (Press CTRL+C to stop)", addr);
    let shutdown_config = ShutdownConfig {
        graceful_timeout: tokio::time::Duration::from_secs(
            app_state.config.server.graceful_shutdown_seconds,
        ),
        enable_ctrl_c: true,
        enable_signal: false,
        enable_custom: app_state.config.server.enable_custom_shutdown,
    };
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown_config))
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    start().await?;

    Ok(())
}
