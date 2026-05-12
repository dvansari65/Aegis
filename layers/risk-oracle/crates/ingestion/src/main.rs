use shock_absorber_ingestion::{AppConfig, DataIngestionService, IngestionError};
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), IngestionError> {
    init_tracing();

    let config = AppConfig::from_env()?;
    let service = DataIngestionService::new(config);

    service.run().await
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .json()
        .init();
}
