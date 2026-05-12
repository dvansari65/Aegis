use std::{env, num::NonZeroU64, time::Duration};

use crate::error::KeeperError;

#[derive(Debug, Clone)]
pub struct KeeperConfig {
    pub poll_interval: Duration,
}

impl KeeperConfig {
    pub fn from_env() -> Result<Self, KeeperError> {
        let poll_interval_ms = env::var("CIRCUIT_BREAKER_KEEPER_INTERVAL_MS")
            .unwrap_or_else(|_| "2000".to_owned())
            .parse::<NonZeroU64>()
            .map_err(|error| {
                KeeperError::Config(format!(
                    "invalid CIRCUIT_BREAKER_KEEPER_INTERVAL_MS value: {error}"
                ))
            })?;

        Ok(Self {
            poll_interval: Duration::from_millis(poll_interval_ms.get()),
        })
    }
}
