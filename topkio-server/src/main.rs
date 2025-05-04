pub mod error;
pub mod handler;
pub mod server;
pub mod state;

use topkio_core::config::load_config;
use crate::server::start_server;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let config = load_config("topkio.toml").expect("Failed to load config");

    // 启动服务器
    if let Err(e) = start_server(config).await {
        eprintln!("Server error: {}", e);
    }
}