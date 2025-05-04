use crate::server::start_server;
use tracing_subscriber;
use topkio_config::{ServerConfig, TopkioConfig};

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let config: ServerConfig = TopkioConfig::parse_section("path", "server")?;

    // 启动服务器
    if let Err(e) = start_server(config).await {
        eprintln!("Server error: {}", e);
    }
}