//! Derive circuit-breaker-style UI fields from the same engines as the keeper (`plan.rs`),
//! without building an on-chain instruction.

use shock_absorber_circuit_breaker::adaptive_fees::AdaptiveFeeEngine;
use shock_absorber_circuit_breaker::policy::CircuitBreakerPolicyEngine;
use shock_absorber_circuit_breaker::throttling::WithdrawalThrottleEngine;
use shock_absorber_circuit_breaker::types::{
    ControlAction, DepegProbability, LiquidityHealth, PolicyContext, ProtectionMode,
    RiskOracleSnapshot,
};

#[derive(Debug, Clone)]
pub struct DerivedCbView {
    pub protection_mode: String,
    pub dynamic_fees_enabled: bool,
    pub withdrawal_throttling_active: bool,
    pub liquidity_routing_active: bool,
    pub toxic_routing_restricted: bool,
}

pub fn liquidity_health_label(h: LiquidityHealth) -> String {
    match h {
        LiquidityHealth::Healthy => "Healthy",
        LiquidityHealth::Watch => "Watch",
        LiquidityHealth::Stressed => "Stressed",
        LiquidityHealth::Critical => "Critical",
        LiquidityHealth::Severe => "Severe",
    }
    .to_owned()
}

pub fn depeg_probability_label(d: DepegProbability) -> String {
    match d {
        DepegProbability::Low => "LOW",
        DepegProbability::Medium => "MEDIUM",
        DepegProbability::High => "HIGH",
        DepegProbability::VeryHigh => "VERY_HIGH",
    }
    .to_owned()
}

fn protection_mode_label(m: ProtectionMode) -> String {
    match m {
        ProtectionMode::Normal => "NORMAL",
        ProtectionMode::Watch => "WATCH",
        ProtectionMode::PanicProtection => "PANIC PROTECTION",
        ProtectionMode::Recovery => "RECOVERY",
    }
    .to_owned()
}

pub fn derive_cb_view(
    chain_slot: u64,
    snapshot: RiskOracleSnapshot,
    previous_mode: ProtectionMode,
) -> DerivedCbView {
    let decision = CircuitBreakerPolicyEngine::with_default_thresholds().evaluate(
        snapshot,
        PolicyContext {
            previous_mode,
            current_slot: chain_slot,
        },
    );

    let fee = AdaptiveFeeEngine::with_default_config().compute(&decision);
    let throttle = WithdrawalThrottleEngine::with_default_config().compute(&decision);
    let throttle_pct_raw = (throttle.max_withdrawal_bps_per_window / 100).min(100_u16);
    let throttle_pct = match u8::try_from(throttle_pct_raw) {
        Ok(v) => v,
        Err(_) => 100,
    };

    let toxic_routing_restricted = decision
        .actions
        .iter()
        .any(|a| matches!(a, ControlAction::RestrictToxicRoutes));
    let liquidity_routing_active = decision
        .actions
        .iter()
        .any(|a| matches!(a, ControlAction::RebalanceLiquidity));

    DerivedCbView {
        protection_mode: protection_mode_label(decision.mode),
        dynamic_fees_enabled: fee.fee_bps > 0,
        withdrawal_throttling_active: throttle_pct < 100,
        liquidity_routing_active,
        toxic_routing_restricted,
    }
}
