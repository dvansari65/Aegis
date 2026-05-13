use tokio::time::{MissedTickBehavior, interval};
use tracing::{info, warn};
use shock_absorber_risk_oracle::signals::MarketSignals;
use shock_absorber_risk_oracle::scoring::calculate_stress_score;
use shock_absorber_risk_oracle::liquidity::calculate_liquidity_health;
use shock_absorber_risk_oracle::depeg::calculate_depeg_probability;
use shock_absorber_risk_oracle::types::{LiquidityHealth, DepegProbabilityBand};

use crate::{config::PublisherConfig, error::PublisherError};

fn fetch_current_slot(rpc_url: &str) -> u64 {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1u64,
        "method": "getSlot",
        "params": [{ "commitment": "processed" }]
    });

    ureq::post(rpc_url)
        .send_json(body)
        .ok()
        .and_then(|resp| resp.into_json::<serde_json::Value>().ok())
        .and_then(|v| v.get("result").and_then(|r| r.as_u64()))
        .unwrap_or(0)
}

pub struct PublisherWorker {
    config: PublisherConfig,
}

impl PublisherWorker {
    pub fn new(config: PublisherConfig) -> Self {
        Self { config }
    }

    pub async fn run(self) -> Result<(), PublisherError> {
        let mut ticker = interval(self.config.poll_interval);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    // Simulate receiving signals from the ingestion layer
                    // In a production setup, this reads from the shared data sink
                    let mock_signals = MarketSignals {
                        withdrawal_volume_24h: 5_000_000.0,
                        pool_imbalance_ratio: 0.76, // High imbalance
                        oracle_price_divergence_bps: 120, // 1.2% divergence
                        slippage_bps: 150,
                        whale_exit_volume: 2_000_000.0,
                    };

                    let score = calculate_stress_score(&mock_signals);
                    let health = calculate_liquidity_health(&mock_signals);
                    let depeg = calculate_depeg_probability(&mock_signals);
                    
                    info!(
                        "Calculated Risk: Score={}, Health={:?}, Depeg={:?}", 
                        score, health, depeg
                    );

                    // Map enums to u8 for the on-chain instruction format
                    let health_u8 = match health {
                        LiquidityHealth::Healthy => 0,
                        LiquidityHealth::Watch => 1,
                        LiquidityHealth::Stressed => 2,
                        LiquidityHealth::Critical => 3,
                        LiquidityHealth::Severe => 4,
                    };

                    let depeg_u8 = match depeg {
                        DepegProbabilityBand::Low => 0,
                        DepegProbabilityBand::Medium => 1,
                        DepegProbabilityBand::High => 2,
                        DepegProbabilityBand::VeryHigh => 3,
                    };

                    let slot = self
                        .config
                        .solana_rpc_url
                        .as_deref()
                        .map(fetch_current_slot)
                        .unwrap_or(0);

                    // [tag=UpdateRisk, score, health, depeg] + optional 8-byte `updated_at_slot` (LE)
                    let mut instruction_payload = vec![1u8, score, health_u8, depeg_u8];
                    if slot > 0 {
                        instruction_payload.extend_from_slice(&slot.to_le_bytes());
                    }

                    warn!(
                        "SIMULATED ON-CHAIN TX PAYLOAD (len={}): {:?}",
                        instruction_payload.len(),
                        instruction_payload
                    );
                }
                result = tokio::signal::ctrl_c() => {
                    result?;
                    info!("publisher shutdown signal received");
                    return Ok(());
                }
            }
        }
    }
}
