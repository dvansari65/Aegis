mod config;
mod error;
mod keeper;

use config::KeeperConfig;
use error::KeeperError;
use keeper::KeeperWorker;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), KeeperError> {
    init_tracing();

    let config = KeeperConfig::from_env()?;
    let worker = KeeperWorker::new(config);

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
