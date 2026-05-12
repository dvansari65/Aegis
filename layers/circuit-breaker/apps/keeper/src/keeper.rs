use tokio::time::{MissedTickBehavior, interval};
use tracing::info;

use crate::{config::KeeperConfig, error::KeeperError};

pub struct KeeperWorker {
    config: KeeperConfig,
}

impl KeeperWorker {
    pub fn new(config: KeeperConfig) -> Self {
        Self { config }
    }

    pub async fn run(self) -> Result<(), KeeperError> {
        let mut ticker = interval(self.config.poll_interval);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    // The real keeper will read risk oracle state and publish
                    // authorized circuit breaker policy updates.
                    info!("keeper tick");
                }
                result = tokio::signal::ctrl_c() => {
                    result?;
                    info!("keeper shutdown signal received");
                    return Ok(());
                }
            }
        }
    }
}
