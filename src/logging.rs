use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup(log: String) -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}
