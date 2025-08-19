use std::{fs::File, io::{Read, Write}};

use anyhow::Result;
use flutter_rust_bridge::frb;
use tracing::{info, level_filters::LevelFilter};
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
pub async fn tokenize(path: &str) -> Result<()> {
    tracing::info!("Tokenize");
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    info!("{}", data.len());
    let mid = (data.len() as f64 * 0.9) as usize;
    let (train, value) = data.split_at(mid);
    crate::tokenize::tiktoken(train)?;
    crate::tokenize::tiktoken(value)?;

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
