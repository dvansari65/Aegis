mod config;
mod error;
mod publisher;

use config::PublisherConfig;
use error::PublisherError;
use publisher::PublisherWorker;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), PublisherError> {
    init_tracing();

    let config = PublisherConfig::from_env()?;
    let worker = PublisherWorker::new(config);

    worker.run().await
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .json()
        .init();
}
