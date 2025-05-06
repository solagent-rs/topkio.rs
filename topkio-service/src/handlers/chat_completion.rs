use axum::{Json, extract::State};
use std::sync::Arc;
use topkio_core::models::{ChatCompletionRequest, ChatCompletionResponse};
use crate::AppState;
use crate::ApiError;

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

pub async fn handle_chat_completion(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, ApiError> {
    let model_id = ModelIdentifier::parse(&request.model)?;
    println!("Received chat completion request for model: {}", request.model);
    
    let backend_name = model_id.backend;
    let model_name = model_id.model_name;

    let backend = state.backends.get(&backend_name)
        .ok_or_else(|| ApiError::BackendNotConfigured(backend_name.to_string()))?;

    // if let Some(supported) = &state.config.backends[&backend_name].supported_models {
    //     if !supported.contains(&model_name.to_string()) {
    //         return Err(ApiError::UnsupportedModel(model_name.to_string()));
    //     }
    // }

    let response = backend.chat_completion(&model_name, request.messages, request.stream)
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))?;

    Ok(Json(response))
}