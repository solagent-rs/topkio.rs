

mod backends;
mod config;
mod models;
mod error;
use error::ApiError;

use axum::{Router, routing::post, Json, extract::State};
use std::{sync::Arc, collections::HashMap};
use crate::{config::GatewayConfig, backends::backend::Backend};
use crate::models::{ChatCompletionRequest, ChatCompletionResponse};

struct AppState {
    backends: HashMap<String, Arc<dyn Backend>>,
    config: GatewayConfig,
}

async fn initialize_backends(
    config: &GatewayConfig,
) -> anyhow::Result<HashMap<String, Arc<dyn Backend>>> {
    let mut backends: HashMap<String, Arc<dyn Backend>> = HashMap::new();

    // Ollama (optional)
    if let Ok(ollama_cfg) = config.get_backend("ollama") {
        let ollama_backend = backends::ollama::OllamaBackend::new(
            ollama_cfg.base_url.clone(),
        );
        ollama_backend.health_check().await?;
        backends.insert("ollama".to_string(), Arc::new(ollama_backend));
    }
    
    Ok(backends)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = GatewayConfig::load_checked("config/default.toml")?;
    let backends = initialize_backends(&config).await?;
    
    let state = Arc::new(AppState { backends, config });

    let app = Router::new()
        .route("/chat/completions", post(handle_chat_completion))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}

#[derive(Debug)]
pub struct ModelIdentifier {
    pub backend: String,
    pub model_name: String,
}

impl ModelIdentifier {
    pub fn parse(model_str: &str) -> Result<Self, ApiError> {
        let (backend, model_name) = model_str.split_once(':')
            .ok_or_else(|| ApiError::InvalidModelFormat(model_str.to_string()))?;

        if backend.is_empty() || model_name.is_empty() {
            return Err(ApiError::InvalidModelFormat(model_str.to_string()));
        }

        Ok(Self {
            backend: backend.to_lowercase(), // Normalize backend name
            model_name: model_name.trim().to_string(), // Trim whitespace
        })
    }
}

async fn handle_chat_completion(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, ApiError> {
    let model_id = ModelIdentifier::parse(&request.model)?;
    let backend_name = model_id.backend;
    let model_name = model_id.model_name;

    let backend = state.backends.get(&backend_name)
        .ok_or_else(|| ApiError::BackendNotConfigured(backend_name.to_string()))?;

    if let Some(supported) = &state.config.backends[&backend_name].supported_models {
        if !supported.contains(&model_name.to_string()) {
            return Err(ApiError::UnsupportedModel(model_name.to_string()));
        }
    }

    let response = backend.chat_completion(&model_name, request.messages)
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))?;

    Ok(Json(response))
}