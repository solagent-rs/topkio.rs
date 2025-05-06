use tokio::sync::oneshot;
use std::time::Duration;

pub struct ShutdownConfig {
    pub graceful_timeout: Duration,
    pub enable_ctrl_c: bool,
    pub enable_signal: bool,
    pub enable_custom: bool,
}

impl Default for ShutdownConfig {
    fn default() -> Self {
        Self {
            graceful_timeout: Duration::from_secs(30),
            enable_ctrl_c: true,
            enable_signal: true,
            enable_custom: false,
        }
    }
}

pub async fn shutdown_signal(config: ShutdownConfig) -> oneshot::Receiver<()> {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    
    tokio::spawn(async move {
        let mut sigterm = tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate()
        ).expect("Failed to install SIGTERM handler");

        tokio::select! {
            _ = async {
                if config.enable_ctrl_c {
                    tokio::signal::ctrl_c()
                        .await
                        .expect("Failed to install CTRL+C handler");
                    println!("Received CTRL+C, initiating shutdown...");
                }
            } => {},
            _ = async {
                if config.enable_signal {
                    sigterm.recv().await;
                    println!("Received SIGTERM, initiating shutdown...");
                }
            } => {},
            _ = async {
                if config.enable_custom {
                    // Custom shutdown trigger (e.g., from health check)
                    tokio::time::sleep(Duration::from_secs(3600)).await; // Example
                    println!("Custom shutdown trigger activated");
                }
            } => {},
        };

        // Notify server to shutdown
        let _ = shutdown_tx.send(());
        
        // Force shutdown if graceful period expires
        tokio::time::sleep(config.graceful_timeout).await;
        println!("Graceful shutdown period expired, forcing exit");
        std::process::exit(0);
    });

    shutdown_rx
}