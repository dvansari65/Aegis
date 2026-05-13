//! Deterministic **risk oracle account → circuit breaker policy → `UpdatePolicy` ix** pipeline.
//! Used by [`crate::keeper::KeeperWorker`] after RPC fetch and by integration tests (no network).

use std::convert::TryInto;

use shock_absorber_circuit_breaker::risk_oracle_feed;
use shock_absorber_circuit_breaker::types::{ProtectionMode, RiskOracleSnapshot};

use crate::cb_account;
use crate::error::KeeperError;
use crate::plan::{self, PolicyWireSummary};

/// Output of one policy tick (for logging and tests).
#[derive(Debug, Clone)]
pub struct PolicyTickOutput {
    pub wire: PolicyWireSummary,
    pub snapshot: RiskOracleSnapshot,
    pub previous_mode: ProtectionMode,
}

/// Run one policy tick from raw account data (same logic as a keeper RPC poll, without HTTP).
pub fn run_policy_tick_from_accounts(
    chain_slot: u64,
    risk_account_data: &[u8],
    cb_account_data: &[u8],
    oracle_confidence: u8,
) -> Result<PolicyTickOutput, KeeperError> {
    let snapshot = risk_oracle_feed::risk_state_to_snapshot(
        risk_account_data,
        oracle_confidence,
        chain_slot,
    )
    .map_err(|e| KeeperError::RiskOracle(e.to_string()))?;

    let previous_mode = cb_account::decode_circuit_breaker_state(cb_account_data)
        .map(|h| h.mode)
        .unwrap_or(ProtectionMode::Normal);

    let wire = plan::evaluate_and_plan(chain_slot, snapshot, previous_mode);

    Ok(PolicyTickOutput {
        wire,
        snapshot,
        previous_mode,
    })
}

/// Decode keeper-produced `UpdatePolicy` bytes (must match on-chain program unpack).
pub fn decode_update_policy_ix(data: &[u8]) -> Option<(u8, u16, u8, bool, u64)> {
    let (&tag, rest) = data.split_first()?;
    if tag != 1 || rest.len() < 13 {
        return None;
    }
    let mode = rest[0];
    let adaptive_fee_bps = u16::from_le_bytes([rest[1], rest[2]]);
    let withdrawal_throttle_pct = rest[3];
    let toxic_routing_restricted = rest[4] != 0;
    let current_slot = u64::from_le_bytes(rest[5..13].try_into().ok()?);
    Some((
        mode,
        adaptive_fee_bps,
        withdrawal_throttle_pct,
        toxic_routing_restricted,
        current_slot,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use shock_absorber_circuit_breaker::types::{DecisionReason, ProtectionMode};

    fn mock_risk_account(stress: u8, health: u8, depeg: u8, oracle_slot: u64) -> [u8; 48] {
        let mut data = [0u8; 48];
        data[0] = 1;
        data[1] = stress;
        data[2] = health;
        data[3] = depeg;
        data[8..16].copy_from_slice(&oracle_slot.to_le_bytes());
        data
    }

    fn mock_cb_account(mode_byte: u8) -> [u8; 45] {
        let mut data = [0u8; 45];
        data[32] = mode_byte;
        data
    }

    #[test]
    fn e2e_healthy_risk_normal_cb_yields_normal_and_low_fee() {
        let risk = mock_risk_account(20, 0, 0, 1_000);
        let cb = mock_cb_account(ProtectionMode::Normal as u8);
        let slot = 1_200u64;

        let out = run_policy_tick_from_accounts(slot, &risk, &cb, 90).expect("tick");
        let wire = &out.wire;

        assert_eq!(wire.mode, ProtectionMode::Normal);
        assert_eq!(wire.reason, DecisionReason::HealthyMarket);
        assert!(!wire.toxic_routing_restricted);

        let decoded = decode_update_policy_ix(
            &hex_to_bytes(&wire.update_policy_instruction_hex).expect("hex"),
        )
        .expect("decode ix");
        assert_eq!(decoded.0, ProtectionMode::Normal as u8);
        assert_eq!(decoded.4, slot);
    }

    #[test]
    fn e2e_high_stress_triggers_panic_and_restricts_routes() {
        let risk = mock_risk_account(91, 0, 0, 1_000);
        let cb = mock_cb_account(ProtectionMode::Normal as u8);
        let slot = 1_200u64;

        let out = run_policy_tick_from_accounts(slot, &risk, &cb, 90).expect("tick");
        let wire = &out.wire;

        assert_eq!(wire.mode, ProtectionMode::PanicProtection);
        assert!(wire.toxic_routing_restricted);
        assert_eq!(
            wire.reason,
            DecisionReason::PanicThresholdBreached
        );

        let raw = hex_to_bytes(&wire.update_policy_instruction_hex).expect("hex");
        let decoded = decode_update_policy_ix(&raw).expect("decode");
        assert_eq!(decoded.0, ProtectionMode::PanicProtection as u8);
        assert!(decoded.3);
    }

    #[test]
    fn e2e_invalid_risk_account_returns_error() {
        let bad = [0u8; 48];
        let cb = mock_cb_account(0);
        let err = run_policy_tick_from_accounts(100, &bad, &cb, 90).unwrap_err();
        assert!(matches!(err, KeeperError::RiskOracle(_)));
    }

    #[test]
    fn e2e_short_cb_account_falls_back_to_normal_previous_mode() {
        let risk = mock_risk_account(91, 0, 0, 1_000);
        let short_cb = [0u8; 10];
        let wire = run_policy_tick_from_accounts(1_200, &risk, &short_cb, 90)
            .expect("tick")
            .wire;
        assert_eq!(wire.mode, ProtectionMode::PanicProtection);
    }

    #[test]
    fn e2e_ix_roundtrip_matches_encoded_fields() {
        let risk = mock_risk_account(60, 0, 0, 500);
        let cb = mock_cb_account(ProtectionMode::Normal as u8);
        let slot = 9_999u64;
        let out = run_policy_tick_from_accounts(slot, &risk, &cb, 90).expect("tick");
        let wire = &out.wire;

        let raw = hex_to_bytes(&wire.update_policy_instruction_hex).expect("hex");
        let (mode, fee_bps, throttle_pct, toxic, sl) = decode_update_policy_ix(&raw).expect("dec");
        assert_eq!(mode, wire.mode as u8);
        assert_eq!(fee_bps, wire.adaptive_fee_bps);
        assert_eq!(throttle_pct, wire.withdrawal_throttle_pct);
        assert_eq!(toxic, wire.toxic_routing_restricted);
        assert_eq!(sl, slot);
    }

    fn hex_to_bytes(hex: &str) -> Option<Vec<u8>> {
        if hex.len() % 2 != 0 {
            return None;
        }
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
            .collect()
    }
}
