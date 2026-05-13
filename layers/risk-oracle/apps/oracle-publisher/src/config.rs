use std::{env, num::NonZeroU64, time::Duration};

use crate::error::PublisherError;

#[derive(Debug, Clone)]
pub struct PublisherConfig {
    pub poll_interval: Duration,
    /// Optional JSON-RPC URL (e.g. `http://127.0.0.1:8899`) used to append `updated_at_slot`
    /// to risk oracle update instructions.
    pub solana_rpc_url: Option<String>,
}

impl PublisherConfig {
    pub fn from_env() -> Result<Self, PublisherError> {
        let poll_interval_ms = env::var("RISK_ORACLE_PUBLISHER_INTERVAL_MS")
            .unwrap_or_else(|_| "2000".to_owned())
            .parse::<NonZeroU64>()
            .map_err(|error| {
                PublisherError::Config(format!(
                    "invalid RISK_ORACLE_PUBLISHER_INTERVAL_MS value: {error}"
                ))
            })?;

        let solana_rpc_url = env::var("SOLANA_RPC_URL").ok().filter(|s| !s.is_empty());

        Ok(Self {
            poll_interval: Duration::from_millis(poll_interval_ms.get()),
            solana_rpc_url,
        })
    }
}
