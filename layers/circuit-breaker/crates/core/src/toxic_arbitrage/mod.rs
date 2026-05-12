//! Defines responses to exploitative routing and liquidity extraction patterns.

use serde::{Deserialize, Serialize};

use crate::types::{
    CircuitBreakerDecision, DepegProbability, LiquidityHealth, PanicType, ProtectionMode,
    ToxicArbitrageLevel, ToxicArbitragePolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToxicArbitrageConfig {
    pub observe_max_route_imbalance_bps: u16,
    pub guarded_max_route_imbalance_bps: u16,
    pub restricted_max_route_imbalance_bps: u16,
    pub emergency_max_route_imbalance_bps: u16,
    pub guarded_delay_slots: u64,
    pub restricted_delay_slots: u64,
    pub emergency_delay_slots: u64,
}

impl Default for ToxicArbitrageConfig {
    fn default() -> Self {
        Self {
            observe_max_route_imbalance_bps: 2_500,
            guarded_max_route_imbalance_bps: 1_500,
            restricted_max_route_imbalance_bps: 750,
            emergency_max_route_imbalance_bps: 300,
            guarded_delay_slots: 10,
            restricted_delay_slots: 30,
            emergency_delay_slots: 75,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ToxicArbitrageEngine {
    config: ToxicArbitrageConfig,
}

impl ToxicArbitrageEngine {
    pub fn new(config: ToxicArbitrageConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(ToxicArbitrageConfig::default())
    }

    pub fn compute(&self, decision: &CircuitBreakerDecision) -> ToxicArbitragePolicy {
        match decision.mode {
            ProtectionMode::Normal => ToxicArbitragePolicy {
                mode: decision.mode,
                level: ToxicArbitrageLevel::Off,
                execution_delay_slots: 0,
                max_route_imbalance_bps: self.config.observe_max_route_imbalance_bps,
                restrict_same_slot_round_trips: false,
                require_mev_protected_path: false,
            },
            ProtectionMode::Watch => ToxicArbitragePolicy {
                mode: decision.mode,
                level: ToxicArbitrageLevel::Observe,
                execution_delay_slots: 0,
                max_route_imbalance_bps: self.config.observe_max_route_imbalance_bps,
                restrict_same_slot_round_trips: true,
                require_mev_protected_path: false,
            },
            ProtectionMode::Recovery => ToxicArbitragePolicy {
                mode: decision.mode,
                level: ToxicArbitrageLevel::Guarded,
                execution_delay_slots: self.config.guarded_delay_slots,
                max_route_imbalance_bps: self.config.guarded_max_route_imbalance_bps,
                restrict_same_slot_round_trips: true,
                require_mev_protected_path: true,
            },
            ProtectionMode::PanicProtection => self.panic_policy(decision),
        }
    }

    fn panic_policy(&self, decision: &CircuitBreakerDecision) -> ToxicArbitragePolicy {
        let emergency = requires_emergency_defense(decision);

        ToxicArbitragePolicy {
            mode: decision.mode,
            level: if emergency {
                ToxicArbitrageLevel::Emergency
            } else {
                ToxicArbitrageLevel::Restricted
            },
            execution_delay_slots: if emergency {
                self.config.emergency_delay_slots
            } else {
                self.config.restricted_delay_slots
            },
            max_route_imbalance_bps: if emergency {
                self.config.emergency_max_route_imbalance_bps
            } else {
                self.config.restricted_max_route_imbalance_bps
            },
            restrict_same_slot_round_trips: true,
            require_mev_protected_path: true,
        }
    }
}

fn requires_emergency_defense(decision: &CircuitBreakerDecision) -> bool {
    matches!(
        decision.snapshot.panic_type,
        PanicType::PossibleManipulation | PanicType::OracleDislocation
    ) || matches!(decision.snapshot.liquidity_health, LiquidityHealth::Severe)
        || matches!(
            decision.snapshot.depeg_probability,
            DepegProbability::VeryHigh
        )
        || decision.snapshot.stress_score >= 95
}

#[cfg(test)]
mod tests {
    use super::ToxicArbitrageEngine;
    use crate::{
        policy::CircuitBreakerPolicyEngine,
        types::{
            DepegProbability, LiquidityHealth, PanicType, PolicyContext, ProtectionMode,
            RiskOracleSnapshot, ToxicArbitrageLevel,
        },
    };

    fn snapshot() -> RiskOracleSnapshot {
        RiskOracleSnapshot {
            stress_score: 20,
            liquidity_health: LiquidityHealth::Healthy,
            depeg_probability: DepegProbability::Low,
            panic_type: PanicType::None,
            confidence: 90,
            last_updated_slot: 1_000,
        }
    }

    fn decision(
        snapshot: RiskOracleSnapshot,
        previous_mode: ProtectionMode,
    ) -> crate::types::CircuitBreakerDecision {
        CircuitBreakerPolicyEngine::with_default_thresholds().evaluate(
            snapshot,
            PolicyContext {
                previous_mode,
                current_slot: 1_200,
            },
        )
    }

    #[test]
    fn turns_off_defense_in_normal_mode() {
        let engine = ToxicArbitrageEngine::with_default_config();

        let policy = engine.compute(&decision(snapshot(), ProtectionMode::Normal));

        assert_eq!(policy.level, ToxicArbitrageLevel::Off);
        assert_eq!(policy.execution_delay_slots, 0);
        assert!(!policy.require_mev_protected_path);
    }

    #[test]
    fn observes_round_trips_in_watch_mode() {
        let engine = ToxicArbitrageEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.stress_score = 60;

        let policy = engine.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.level, ToxicArbitrageLevel::Observe);
        assert!(policy.restrict_same_slot_round_trips);
        assert!(!policy.require_mev_protected_path);
    }

    #[test]
    fn restricts_routes_in_panic_mode() {
        let engine = ToxicArbitrageEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.stress_score = 90;

        let policy = engine.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.level, ToxicArbitrageLevel::Restricted);
        assert_eq!(policy.execution_delay_slots, 30);
        assert_eq!(policy.max_route_imbalance_bps, 750);
        assert!(policy.require_mev_protected_path);
    }

    #[test]
    fn escalates_to_emergency_for_possible_manipulation() {
        let engine = ToxicArbitrageEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.panic_type = PanicType::PossibleManipulation;

        let policy = engine.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.level, ToxicArbitrageLevel::Emergency);
        assert_eq!(policy.execution_delay_slots, 75);
        assert_eq!(policy.max_route_imbalance_bps, 300);
    }

    #[test]
    fn keeps_guardrails_during_recovery() {
        let engine = ToxicArbitrageEngine::with_default_config();

        let policy = engine.compute(&decision(snapshot(), ProtectionMode::PanicProtection));

        assert_eq!(policy.mode, ProtectionMode::Recovery);
        assert_eq!(policy.level, ToxicArbitrageLevel::Guarded);
        assert!(policy.require_mev_protected_path);
    }
}
