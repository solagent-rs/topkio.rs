use axum::{extract::State, Json};
use dashmap::DashMap;
use std::sync::Arc;

use crate::{
    models::{CompletionRequest, CompletionResponse, GatewayError},
    providers::{Provider, ProviderMap},
};

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub providers: ProviderMap,
    // Add other shared state here
}

pub async fn handle_completion(
    State(state): State<SharedState>,
    Json(request): Json<CompletionRequest>,
) -> Result<Json<CompletionResponse>, GatewayError> {
    // Select appropriate provider based on model
    let provider = state
        .providers
        // .select_provider(&request.model)
        .ok_or(GatewayError::InvalidRequest)?;

    // Forward request to provider
    let response = provider
        .create_completion(request)
        .await
        .map_err(GatewayError::ProviderError)?;

    Ok(Json(response))
}