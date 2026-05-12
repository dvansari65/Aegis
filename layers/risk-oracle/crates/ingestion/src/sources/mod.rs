mod pyth;
mod solana_rpc;
mod dex_pools;
mod whale_tracker;

use async_trait::async_trait;

use crate::{config::AppConfig, error::IngestionError, error::SourceError, models::EventEnvelope};

pub use pyth::PythHermesSource;
pub use solana_rpc::SolanaRpcSource;
pub use dex_pools::DexPoolSource;
pub use whale_tracker::WhaleTrackerSource;

#[async_trait]
pub trait Source: Send + Sync {
    fn name(&self) -> &'static str;
    // Sources own the translation from raw upstream responses into normalized
    // ingestion events.
    async fn fetch(&self) -> Result<Vec<EventEnvelope>, SourceError>;
}

pub async fn create_sources(config: &AppConfig) -> Result<Vec<Box<dyn Source>>, IngestionError> {
    let mut sources: Vec<Box<dyn Source>> = Vec::new();

    if !config.monitored_accounts.is_empty() {
        sources.push(Box::new(SolanaRpcSource::new(
            config.rpc_url.clone(),
            config.monitored_accounts.clone(),
        )));
        
        // Use the same monitored accounts for DEX pool balances
        sources.push(Box::new(DexPoolSource::new(
            config.rpc_url.clone(),
            config.monitored_accounts.clone(),
        )));
    }

    if !config.pyth_feed_ids.is_empty() {
        // Pyth feed IDs are meaningless without the Hermes endpoint that serves
        // the corresponding price updates.
        let hermes_url = config.pyth_hermes_url.clone().ok_or_else(|| {
            IngestionError::InvalidConfig(
                "SFA_PYTH_HERMES_URL is required when SFA_PYTH_FEED_IDS is configured".to_owned(),
            )
        })?;

        sources.push(Box::new(PythHermesSource::new(
            hermes_url,
            config.pyth_feed_ids.clone(),
        )));
    }

    // Whale tracker (using a dummy mint for now, like USDC)
    sources.push(Box::new(WhaleTrackerSource::new(
        config.rpc_url.clone(),
        vec!["EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string()],
    )));

    Ok(sources)
}
