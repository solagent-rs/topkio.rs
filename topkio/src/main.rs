use axum::{routing::post, Router};
use std::net::SocketAddr;
use tracing::info;

mod config;
mod models;
mod rate_limit;
mod router;
mod providers;

use config::GatewayConfig;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = GatewayConfig::load().expect("Failed to load config");

    // Build router with middleware
    let app = Router::new()
        .route("/v1/completions", post(router::handle_completion))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower::ServiceBuilder::new()
            .layer(axum::middleware::from_fn(rate_limit::rate_limit_middleware)));

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}