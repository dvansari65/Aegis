//! Decode on-chain risk oracle account data into [`crate::types::RiskOracleSnapshot`].
//!
//! Layout matches `layers/risk-oracle/programs/risk-oracle/src/state.rs` (`RiskState`, 48 bytes).

use std::fmt;

use crate::types::{DepegProbability, LiquidityHealth, PanicType, RiskOracleSnapshot};

/// Byte length of the risk oracle state account payload.
pub const RISK_STATE_LEN: usize = 48;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RiskFeedError;

impl fmt::Display for RiskFeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid or uninitialized risk oracle account data")
    }
}

/// Map on-chain `liquidity_health` byte (publisher encoding) to the circuit breaker enum.
pub fn liquidity_health_from_u8(v: u8) -> Result<LiquidityHealth, RiskFeedError> {
    match v {
        0 => Ok(LiquidityHealth::Healthy),
        1 => Ok(LiquidityHealth::Watch),
        2 => Ok(LiquidityHealth::Stressed),
        3 => Ok(LiquidityHealth::Critical),
        4 => Ok(LiquidityHealth::Severe),
        _ => Err(RiskFeedError),
    }
}

/// Map on-chain `depeg_probability` byte (publisher encoding) to the circuit breaker enum.
pub fn depeg_probability_from_u8(v: u8) -> Result<DepegProbability, RiskFeedError> {
    match v {
        0 => Ok(DepegProbability::Low),
        1 => Ok(DepegProbability::Medium),
        2 => Ok(DepegProbability::High),
        3 => Ok(DepegProbability::VeryHigh),
        _ => Err(RiskFeedError),
    }
}

/// Decode a risk oracle state account and build a snapshot for policy evaluation.
///
/// `chain_slot` is used when the account has not yet recorded a non-zero `updated_at_slot`
/// (older publishers); recovery cooldown then uses chain time instead of a stuck zero.
pub fn risk_state_to_snapshot(
    data: &[u8],
    oracle_confidence: u8,
    chain_slot: u64,
) -> Result<RiskOracleSnapshot, RiskFeedError> {
    if data.len() < RISK_STATE_LEN {
        return Err(RiskFeedError);
    }
    if data[0] == 0 {
        return Err(RiskFeedError);
    }

    let stress_score = data[1];
    let liquidity_health = liquidity_health_from_u8(data[2])?;
    let depeg_probability = depeg_probability_from_u8(data[3])?;

    let mut slot_bytes = [0u8; 8];
    slot_bytes.copy_from_slice(&data[8..16]);
    let from_account = u64::from_le_bytes(slot_bytes);
    let last_updated_slot = if from_account == 0 {
        chain_slot
    } else {
        from_account
    };

    Ok(RiskOracleSnapshot {
        stress_score,
        liquidity_health,
        depeg_probability,
        panic_type: PanicType::None,
        confidence: oracle_confidence,
        last_updated_slot,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_risk_account_to_snapshot() {
        let mut data = [0u8; RISK_STATE_LEN];
        data[0] = 1;
        data[1] = 42;
        data[2] = 2;
        data[3] = 1;
        data[8..16].copy_from_slice(&999u64.to_le_bytes());

        let s = risk_state_to_snapshot(&data, 88, 1_500).unwrap();
        assert_eq!(s.stress_score, 42);
        assert_eq!(s.liquidity_health, LiquidityHealth::Stressed);
        assert_eq!(s.depeg_probability, DepegProbability::Medium);
        assert_eq!(s.confidence, 88);
        assert_eq!(s.last_updated_slot, 999);
    }

    #[test]
    fn uses_chain_slot_when_account_slot_zero() {
        let mut data = [0u8; RISK_STATE_LEN];
        data[0] = 1;
        data[1] = 10;
        data[2] = 0;
        data[3] = 0;

        let s = risk_state_to_snapshot(&data, 90, 2_000).unwrap();
        assert_eq!(s.last_updated_slot, 2_000);
    }
}
