use shock_absorber_circuit_breaker::adaptive_fees::AdaptiveFeeEngine;
use shock_absorber_circuit_breaker::policy::CircuitBreakerPolicyEngine;
use shock_absorber_circuit_breaker::throttling::WithdrawalThrottleEngine;
use shock_absorber_circuit_breaker::types::{
    ControlAction, DecisionReason, PolicyContext, ProtectionMode, RiskOracleSnapshot,
};

use crate::ix;

#[derive(Debug, Clone)]
pub struct PolicyWireSummary {
    pub mode: ProtectionMode,
    pub adaptive_fee_bps: u16,
    pub withdrawal_throttle_pct: u8,
    pub toxic_routing_restricted: bool,
    pub reason: DecisionReason,
    pub update_policy_instruction_hex: String,
}

pub fn evaluate_and_plan(
    chain_slot: u64,
    snapshot: RiskOracleSnapshot,
    previous_mode: ProtectionMode,
) -> PolicyWireSummary {
    let decision = CircuitBreakerPolicyEngine::with_default_thresholds().evaluate(
        snapshot,
        PolicyContext {
            previous_mode,
            current_slot: chain_slot,
        },
    );

    let fee = AdaptiveFeeEngine::with_default_config().compute(&decision);
    let throttle = WithdrawalThrottleEngine::with_default_config().compute(&decision);
    let throttle_pct = u8::try_from((throttle.max_withdrawal_bps_per_window / 100).min(100))
        .unwrap_or(100);

    let toxic_routing_restricted = decision
        .actions
        .iter()
        .any(|a| matches!(a, ControlAction::RestrictToxicRoutes));

    let raw = ix::encode_update_policy(
        decision.mode,
        fee.fee_bps,
        throttle_pct,
        toxic_routing_restricted,
        chain_slot,
    );

    let update_policy_instruction_hex = raw.iter().map(|b| format!("{b:02x}")).collect();

    PolicyWireSummary {
        mode: decision.mode,
        adaptive_fee_bps: fee.fee_bps,
        withdrawal_throttle_pct: throttle_pct,
        toxic_routing_restricted,
        reason: decision.reason,
        update_policy_instruction_hex,
    }
}
