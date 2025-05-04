pub mod server;
pub mod handler;

use topkio_core::config::load_config;
use crate::server::start_server;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置文件
    let config = load_config("../../../config/topkio.toml")?;
    tracing::info!("Configuration loaded: {:?}", config);

    // 启动服务器
    start_server(config).await?;
    Ok(())
}