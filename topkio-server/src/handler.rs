use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::collections::HashMap;
use topkio_core::config::Config;
use topkio_core::error::TopkioError;
use topkio_core::models::{ChatCompletionRequest, ChatCompletionResponse};
use topkio_core::provider::Provider;
use topkio_providers::select_provider;

pub async fn handle_chat_completion(
    State((config, providers)): State<(Config, HashMap<String, Box<dyn Provider>>)>,
    Json(request): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, TopkioErrorResponse> {
    let provider = select_provider(&providers, &request.model)
        .ok_or_else(|| {
            TopkioErrorResponse(
                StatusCode::BAD_REQUEST,
                json!({ "error": "Unsupported model" }).to_string(),
            )
        })?;

    let response = provider
        .chat_completion(request)
        .await
        .map_err(TopkioErrorResponse::from)?;
    Ok(Json(response))
}

pub struct TopkioErrorResponse(StatusCode, String);

impl From<TopkioError> for TopkioErrorResponse {
    fn from(err: TopkioError) -> Self {
        TopkioErrorResponse(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

impl IntoResponse for TopkioErrorResponse {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}