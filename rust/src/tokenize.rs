use anyhow::Result;
use tracing::info;

pub fn tiktoken(value: &str) -> Result<()> {
    info!("length = {}", value.len());
    Ok(())
}