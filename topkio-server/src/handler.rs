use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use topkio_core::error::TopkioError;
use topkio_core::models::{ChatCompletionRequest, ChatCompletionResponse};
use tracing::{error, info};

use crate::error::TopkioErrorResponse;
use crate::state::AppState;

pub async fn handle_chat_completion(
    State(state): State<AppState>,
    Json(request): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, TopkioErrorResponse> {
    info!("Received chat completion request for model: {}", request.model);

    // 选择提供商
    let provider = state
        .select_provider(&request.model)
        .ok_or_else(|| {
            let msg = format!("Unsupported model: {}", request.model);
            error!("{}", msg);
            TopkioErrorResponse(StatusCode::BAD_REQUEST, json!({ "error": msg }).to_string())
        })?;

    // 调用提供商
    let response = provider
        .chat_completion(request)
        .await
        .map_err(|e| {
            error!("Provider error: {}", e);
            TopkioErrorResponse::from(TopkioError::ProviderError(e))
        })?;

    info!("Chat completion successful for model: {}", response.model);
    Ok(Json(response))
}