use circuit_breaker_keeper::{init_tracing, KeeperConfig, KeeperError, KeeperWorker};

#[tokio::main]
async fn main() -> Result<(), KeeperError> {
    init_tracing();

    let config = KeeperConfig::from_env()?;
    let worker = KeeperWorker::new(config);

    worker.run().await
}
