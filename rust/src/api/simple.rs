use std::{fs::File, io::Write};

use anyhow::Result;
use flutter_rust_bridge::frb;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::{self, format::FmtSpan}, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter, Layer, Registry};

#[frb]
pub async fn download_training_data(path: &str, url: &str) -> Result<()> {
    let rep = reqwest::get(url).await?;
    let body = rep.text().await?;
    let mut file = File::create(path)?;
    write!(file, "{}", body)?;
    Ok(())
}

#[frb]
pub async fn tokenize() -> Result<()> {
    tracing::info!("Tokenize");
    Ok(())
}

type BoxedLayer<S> = Box<dyn Layer<S> + Send + Sync + 'static>;

fn default_layer<S>() -> BoxedLayer<S>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fmt::layer()
        .with_ansi(false)
        .with_span_events(FmtSpan::ACTIVE)
        .compact()
        .boxed()
}

fn env_layer<S>() -> BoxedLayer<S>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .boxed()
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
    let _ = env_logger::builder().try_init();
    let _ = Registry::default()
        .with(default_layer())
        .with(env_layer())
        .try_init();
}
