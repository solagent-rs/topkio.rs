#![allow(unused)]

use tokio::{sync::oneshot, time::Duration};

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

/// Configurable shutdown signal with multiple triggers
pub async fn shutdown_signal(config: ShutdownConfig) {
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        #[cfg(unix)]
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler");

        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("\nReceived CTRL+C, initiating graceful shutdown...");
            },
            _ = async {
                #[cfg(unix)]
                {
                    sigterm.recv().await;
                    println!("Received SIGTERM, initiating graceful shutdown...");
                }
                #[cfg(not(unix))]
                {
                    tokio::time::sleep(Duration::MAX).await;
                }
            } => {},
            _ = async {
                // Custom shutdown trigger (e.g., from health check)
                // Example: shutdown after 1 hour for demonstration
                tokio::time::sleep(Duration::from_secs(3600)).await;
                println!("Custom shutdown trigger activated");
            } => {},
        };

        // Signal the server to shutdown
        let _ = shutdown_tx.send(());

        // Force shutdown if graceful period expires
        tokio::time::sleep(Duration::from_secs(30)).await;
        println!("Graceful shutdown period expired, forcing exit");
        std::process::exit(0);
    });

    // Wait for shutdown signal
    let _ = shutdown_rx.await;
    println!("Shutting down gracefully...");

    // Add any cleanup operations here
    cleanup_resources().await;
}

/// Example resource cleanup function
async fn cleanup_resources() {
    println!("Closing database connections...");
    tokio::time::sleep(Duration::from_secs(1)).await; // Simulate cleanup
    println!("Resources cleaned up successfully");
}
