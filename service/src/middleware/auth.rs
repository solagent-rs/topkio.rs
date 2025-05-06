use axum::{
    http::Request,
    middleware::Next,
    response::Response,
    extract::State,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthParams {
    pub token: String,
}

pub async fn auth_middleware<B>(
    State(app_state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, ApiError> {
    // Extract token from headers
    let token = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    // Validate token (example using app state)
    if !app_state.valid_tokens.contains(token) {
        return Err(ApiError::Unauthorized);
    }

    // Add user data to request extensions
    req.extensions_mut().insert(User { id: 123 });

    Ok(next.run(req).await)
}