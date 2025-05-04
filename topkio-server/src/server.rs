use axum::{routing::post, Router};
use std::net::SocketAddr;
use topkio_core::config::Config;
use topkio_core::error::TopkioError;
use topkio_providers::create_providers;
use topkio_server::handler::handle_chat_completion;

pub async fn start_server(config: Config) -> Result<(), TopkioError> {
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>().unwrap(),
        config.server.port,
    ));

    let providers = create_providers(config.providers.clone());
    let app = Router::new()
        .route("/chat/completions", post(handle_chat_completion))
        .with_state((config, providers));

    tracing::info!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| TopkioError::ProviderError(topkio_providers::error::ProviderError::ProviderError(e.to_string())))?;
    Ok(())
}