use anyhow::Result; 

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Topkio Gateway...");

    topkio_service::start().await?;

    Ok(())
}