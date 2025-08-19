use std::{fs::File, io::Write};

use anyhow::Result;
use flutter_rust_bridge::frb;

#[frb]
pub async fn download_training_data(path: &str, url: &str) -> Result<()> {
    let rep = reqwest::get(url).await?;
    let body = rep.text().await?;
    let mut file = File::create(path)?;
    write!(file, "{}", body)?;
    Ok(())
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
