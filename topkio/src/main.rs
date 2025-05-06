use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    topkio_service::start().await?;

    Ok(())
}
