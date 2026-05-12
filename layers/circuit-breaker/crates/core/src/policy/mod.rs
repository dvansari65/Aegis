use serde::{Deserialize, Serialize};

use crate::recovery::{RecoveryConfig, RecoveryEngine};
use crate::types::{
    CircuitBreakerDecision, ControlAction, DecisionReason, DepegProbability, LiquidityHealth,
    PanicType, PolicyContext, ProtectionMode, RecoveryReason, RiskOracleSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyThresholds {
    pub min_confidence: u8,
    pub watch_stress_score: u8,
    pub panic_stress_score: u8,
    pub recovery_stress_score: u8,
    pub recovery_cooldown_slots: u64,
}

impl Default for PolicyThresholds {
    fn default() -> Self {
        Self {
            min_confidence: 60,
            watch_stress_score: 55,
            panic_stress_score: 85,
            recovery_stress_score: 45,
            recovery_cooldown_slots: 150,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CircuitBreakerPolicyEngine {
    thresholds: PolicyThresholds,
}

impl CircuitBreakerPolicyEngine {
    pub fn new(thresholds: PolicyThresholds) -> Self {
        Self { thresholds }
    }

    pub fn with_default_thresholds() -> Self {
        Self::new(PolicyThresholds::default())
    }

    pub fn evaluate(
        &self,
        snapshot: RiskOracleSnapshot,
        context: PolicyContext,
    ) -> CircuitBreakerDecision {
        let (mode, reason) = self.select_mode(snapshot, context);

        CircuitBreakerDecision {
            mode,
            reason,
            actions: actions_for_mode(mode),
            snapshot,
        }
    }

    fn select_mode(
        &self,
        snapshot: RiskOracleSnapshot,
        context: PolicyContext,
    ) -> (ProtectionMode, DecisionReason) {
        if snapshot.confidence < self.thresholds.min_confidence {
            return (ProtectionMode::Watch, DecisionReason::LowConfidence);
        }

        if is_panic(snapshot, self.thresholds.panic_stress_score) {
            return (
                ProtectionMode::PanicProtection,
                panic_reason(snapshot, self.thresholds.panic_stress_score),
            );
        }

        if context.previous_mode == ProtectionMode::PanicProtection
            || context.previous_mode == ProtectionMode::Recovery
        {
            return self.select_recovery_mode(snapshot, context);
        }

        if snapshot.stress_score >= self.thresholds.watch_stress_score
            || matches!(snapshot.liquidity_health, LiquidityHealth::Watch)
        {
            return (
                ProtectionMode::Watch,
                DecisionReason::WatchThresholdBreached,
            );
        }

        (ProtectionMode::Normal, DecisionReason::HealthyMarket)
    }

    fn select_recovery_mode(
        &self,
        snapshot: RiskOracleSnapshot,
        context: PolicyContext,
    ) -> (ProtectionMode, DecisionReason) {
        let recovery = RecoveryEngine::new(RecoveryConfig {
            recovery_stress_score: self.thresholds.recovery_stress_score,
            recovery_cooldown_slots: self.thresholds.recovery_cooldown_slots,
        })
        .evaluate(snapshot, context);

        let reason = match recovery.reason {
            RecoveryReason::ConditionsMet => DecisionReason::RecoveryConditionsMet,
            _ => DecisionReason::RecoveryStillCoolingDown,
        };

        (recovery.next_mode, reason)
    }
}

fn is_panic(snapshot: RiskOracleSnapshot, panic_stress_score: u8) -> bool {
    snapshot.stress_score >= panic_stress_score
        || matches!(
            snapshot.liquidity_health,
            LiquidityHealth::Critical | LiquidityHealth::Severe
        )
        || matches!(
            snapshot.depeg_probability,
            DepegProbability::High | DepegProbability::VeryHigh
        )
        || matches!(
            snapshot.panic_type,
            PanicType::LiquidityPanic
                | PanicType::OracleDislocation
                | PanicType::SystemicMarketStress
                | PanicType::PossibleManipulation
        )
}

fn panic_reason(snapshot: RiskOracleSnapshot, panic_stress_score: u8) -> DecisionReason {
    if matches!(
        snapshot.liquidity_health,
        LiquidityHealth::Critical | LiquidityHealth::Severe
    ) {
        return DecisionReason::SevereLiquidityStress;
    }

    if matches!(
        snapshot.depeg_probability,
        DepegProbability::High | DepegProbability::VeryHigh
    ) {
        return DecisionReason::HighDepegRisk;
    }

    if snapshot.stress_score >= panic_stress_score {
        return DecisionReason::PanicThresholdBreached;
    }

    DecisionReason::PanicThresholdBreached
}

fn actions_for_mode(mode: ProtectionMode) -> Vec<ControlAction> {
    match mode {
        ProtectionMode::Normal => vec![ControlAction::NoAction],
        ProtectionMode::Watch => vec![ControlAction::CoordinateProtocolResponse],
        ProtectionMode::PanicProtection => vec![
            ControlAction::IncreaseFees,
            ControlAction::ThrottleWithdrawals,
            ControlAction::RebalanceLiquidity,
            ControlAction::RestrictToxicRoutes,
            ControlAction::CoordinateProtocolResponse,
        ],
        ProtectionMode::Recovery => vec![
            ControlAction::IncreaseFees,
            ControlAction::CoordinateProtocolResponse,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{CircuitBreakerPolicyEngine, PolicyThresholds};
    use crate::types::{
        DecisionReason, DepegProbability, LiquidityHealth, PanicType, PolicyContext,
        ProtectionMode, RiskOracleSnapshot,
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

    fn context(previous_mode: ProtectionMode) -> PolicyContext {
        PolicyContext {
            previous_mode,
            current_slot: 1_200,
        }
    }

    #[test]
    fn returns_normal_for_healthy_market() {
        let engine = CircuitBreakerPolicyEngine::with_default_thresholds();

        let decision = engine.evaluate(snapshot(), context(ProtectionMode::Normal));

        assert_eq!(decision.mode, ProtectionMode::Normal);
        assert_eq!(decision.reason, DecisionReason::HealthyMarket);
    }

    #[test]
    fn returns_watch_when_stress_crosses_watch_threshold() {
        let engine = CircuitBreakerPolicyEngine::with_default_thresholds();
        let mut snapshot = snapshot();
        snapshot.stress_score = 60;

        let decision = engine.evaluate(snapshot, context(ProtectionMode::Normal));

        assert_eq!(decision.mode, ProtectionMode::Watch);
        assert_eq!(decision.reason, DecisionReason::WatchThresholdBreached);
    }

    #[test]
    fn returns_watch_when_confidence_is_low() {
        let engine = CircuitBreakerPolicyEngine::with_default_thresholds();
        let mut snapshot = snapshot();
        snapshot.confidence = 40;

        let decision = engine.evaluate(snapshot, context(ProtectionMode::Normal));

        assert_eq!(decision.mode, ProtectionMode::Watch);
        assert_eq!(decision.reason, DecisionReason::LowConfidence);
    }

    #[test]
    fn returns_panic_protection_for_high_stress() {
        let engine = CircuitBreakerPolicyEngine::with_default_thresholds();
        let mut snapshot = snapshot();
        snapshot.stress_score = 91;

        let decision = engine.evaluate(snapshot, context(ProtectionMode::Normal));

        assert_eq!(decision.mode, ProtectionMode::PanicProtection);
        assert_eq!(decision.reason, DecisionReason::PanicThresholdBreached);
    }

    #[test]
    fn returns_panic_protection_for_critical_liquidity() {
        let engine = CircuitBreakerPolicyEngine::with_default_thresholds();
        let mut snapshot = snapshot();
        snapshot.liquidity_health = LiquidityHealth::Critical;

        let decision = engine.evaluate(snapshot, context(ProtectionMode::Normal));

        assert_eq!(decision.mode, ProtectionMode::PanicProtection);
        assert_eq!(decision.reason, DecisionReason::SevereLiquidityStress);
    }

    #[test]
    fn stays_in_panic_until_recovery_cooldown_finishes() {
        let engine = CircuitBreakerPolicyEngine::new(PolicyThresholds {
            recovery_cooldown_slots: 500,
            ..PolicyThresholds::default()
        });
        let snapshot = snapshot();

        let decision = engine.evaluate(
            snapshot,
            PolicyContext {
                previous_mode: ProtectionMode::PanicProtection,
                current_slot: 1_200,
            },
        );

        assert_eq!(decision.mode, ProtectionMode::PanicProtection);
        assert_eq!(decision.reason, DecisionReason::RecoveryStillCoolingDown);
    }

    #[test]
    fn enters_recovery_when_market_has_cooled() {
        let engine = CircuitBreakerPolicyEngine::with_default_thresholds();
        let snapshot = snapshot();

        let decision = engine.evaluate(snapshot, context(ProtectionMode::PanicProtection));

        assert_eq!(decision.mode, ProtectionMode::Recovery);
        assert_eq!(decision.reason, DecisionReason::RecoveryConditionsMet);
    }
}
