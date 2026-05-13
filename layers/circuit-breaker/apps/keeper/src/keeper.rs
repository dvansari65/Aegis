use tokio::time::{MissedTickBehavior, interval};
use tracing::info;

use crate::config::KeeperConfig;
use crate::error::KeeperError;
use crate::rpc;

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
                    self.tick_once()?;
                }
                result = tokio::signal::ctrl_c() => {
                    result?;
                    info!("keeper shutdown signal received");
                    return Ok(());
                }
            }
        }
    }

    fn tick_once(&self) -> Result<(), KeeperError> {
        let slot = rpc::get_slot(&self.config.solana_rpc_url)?;

        let risk_data = rpc::get_account_data(
            &self.config.solana_rpc_url,
            &self.config.risk_oracle_state_pubkey,
        )?;

        let cb_data = rpc::get_account_data(
            &self.config.solana_rpc_url,
            &self.config.circuit_breaker_state_pubkey,
        )?;

        let out = crate::pipeline::run_policy_tick_from_accounts(
            slot,
            &risk_data,
            &cb_data,
            self.config.oracle_confidence,
        )?;

        let wire = &out.wire;
        let snapshot = &out.snapshot;
        let previous_mode = out.previous_mode;

        info!(
            target: "aegis.keeper",
            chain_slot = slot,
            stress = snapshot.stress_score,
            liquidity_health = ?snapshot.liquidity_health,
            depeg = ?snapshot.depeg_probability,
            previous_mode = ?previous_mode,
            next_mode = ?wire.mode,
            reason = ?wire.reason,
            adaptive_fee_bps = wire.adaptive_fee_bps,
            withdrawal_throttle_pct = wire.withdrawal_throttle_pct,
            toxic_routing_restricted = wire.toxic_routing_restricted,
            update_policy_ix_hex = %wire.update_policy_instruction_hex,
            "risk oracle → circuit breaker policy tick (submit UpdatePolicy with authority signer separately)"
        );

        Ok(())
    }
}
