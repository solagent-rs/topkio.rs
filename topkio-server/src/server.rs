use axum::{routing::post, Router};
use std::net::SocketAddr;
use topkio_core::config::Config;
use topkio_core::error::TopkioError;
use tracing::info;

use crate::handler::handle_chat_completion;
use crate::state::AppState;

pub async fn start_server(config: Config) -> Result<(), TopkioError> {
    // 创建应用状态
    let state = AppState::new(config.clone())?;

    // 配置 Axum 路由
    let app = Router::new()
        .route("/chat/completions", post(handle_chat_completion))
        .with_state(state);

    // 启动服务器
    let addr = SocketAddr::from((
        config
            .server
            .host
            .parse::<std::net::IpAddr>()
            .map_err(|e| TopkioError::ConfigError(e.to_string()))?,
        config.server.port,
    ));

    info!("Server running at http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| TopkioError::ServerError(e.to_string()))?;

    Ok(())
}