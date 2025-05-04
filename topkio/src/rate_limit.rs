use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use std::{sync::Arc, time::{Instant, Duration}};
use tower::ServiceBuilder;

use crate::{models::GatewayError, config::GatewayConfig};

#[derive(Clone)]
pub struct RateLimiter {
    config: GatewayConfig,
    counters: Arc<DashMap<String, (u32, Instant)>>,
}

impl RateLimiter {
    pub fn new(config: GatewayConfig) -> Self {
        Self {
            config,
            counters: Arc::new(DashMap::new()),
        }
    }

    pub fn check_limit(&self, api_key: &str) -> Result<(), GatewayError> {
        let mut entry = self.counters.entry(api_key.to_string()).or_insert((0, Instant::now()));
        
        // Reset counter if time window has passed
        if entry.1.elapsed() > Duration::from_secs(60) {
            *entry = (0, Instant::now());
        }

        // Check limit
        if entry.0 >= self.config.rate_limits.requests_per_minute {
            Err(GatewayError::RateLimitExceeded)
        } else {
            *entry = (entry.0 + 1, entry.1);
            Ok(())
        }
    }
}

pub async fn rate_limit_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, impl IntoResponse> {
    // Extract API key from headers
    let api_key = req.headers()
        .get("x-api-key")
        .and_then(|h| h.to_str().ok())
        .ok_or(GatewayError::InvalidRequest)?;

    // Get rate limiter from extensions
    let rate_limiter = req.extensions()
        .get::<RateLimiter>()
        .expect("RateLimiter not found in extensions");

    // Check rate limit
    rate_limiter.check_limit(api_key)?;

    Ok(next.run(req).await)
}