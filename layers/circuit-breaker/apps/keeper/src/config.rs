use std::{env, num::NonZeroU64, time::Duration};

use crate::error::KeeperError;

fn require_env(name: &'static str) -> Result<String, KeeperError> {
    env::var(name)
        .map_err(|_| KeeperError::Config(format!("missing required env var {name}")))
        .and_then(|v| {
            if v.trim().is_empty() {
                Err(KeeperError::Config(format!(
                    "env var {name} must not be empty"
                )))
            } else {
                Ok(v)
            }
        })
}

fn validate_b58(label: &'static str, value: &str) -> Result<(), KeeperError> {
    bs58::decode(value)
        .into_vec()
        .map_err(|e| KeeperError::Config(format!("{label} is not valid base58: {e}")))?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct KeeperConfig {
    pub poll_interval: Duration,
    pub solana_rpc_url: String,
    pub risk_oracle_state_pubkey: String,
    pub circuit_breaker_state_pubkey: String,
    /// Confidence 0–100 passed into [`RiskOracleSnapshot::confidence`] (oracle has no on-chain field yet).
    pub oracle_confidence: u8,
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

        let solana_rpc_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8899".to_owned());

        let risk_oracle_state_pubkey = require_env("RISK_ORACLE_STATE_PUBKEY")?;
        let circuit_breaker_state_pubkey = require_env("CIRCUIT_BREAKER_STATE_PUBKEY")?;

        validate_b58("RISK_ORACLE_STATE_PUBKEY", &risk_oracle_state_pubkey)?;
        validate_b58("CIRCUIT_BREAKER_STATE_PUBKEY", &circuit_breaker_state_pubkey)?;

        let oracle_confidence = env::var("AEGIS_ORACLE_CONFIDENCE")
            .unwrap_or_else(|_| "90".to_owned())
            .parse::<u8>()
            .map_err(|e| {
                KeeperError::Config(format!("invalid AEGIS_ORACLE_CONFIDENCE (0–100): {e}"))
            })?;

        Ok(Self {
            poll_interval: Duration::from_millis(poll_interval_ms.get()),
            solana_rpc_url,
            risk_oracle_state_pubkey,
            circuit_breaker_state_pubkey,
            oracle_confidence,
        })
    }
}
