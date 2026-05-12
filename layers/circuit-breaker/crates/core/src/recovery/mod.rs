//! Defines how emergency controls unwind as stress decreases.

use serde::{Deserialize, Serialize};

use crate::types::{
    DepegProbability, LiquidityHealth, PanicType, PolicyContext, ProtectionMode, RecoveryDecision,
    RecoveryReason, RiskOracleSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryConfig {
    pub recovery_stress_score: u8,
    pub recovery_cooldown_slots: u64,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            recovery_stress_score: 45,
            recovery_cooldown_slots: 150,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecoveryEngine {
    config: RecoveryConfig,
}

impl RecoveryEngine {
    pub fn new(config: RecoveryConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(RecoveryConfig::default())
    }

    pub fn evaluate(
        &self,
        snapshot: RiskOracleSnapshot,
        context: PolicyContext,
    ) -> RecoveryDecision {
        if !matches!(
            context.previous_mode,
            ProtectionMode::PanicProtection | ProtectionMode::Recovery
        ) {
            return RecoveryDecision {
                can_exit_panic: false,
                next_mode: context.previous_mode,
                cooldown_remaining_slots: 0,
                reason: RecoveryReason::NotInEmergencyMode,
            };
        }

        if snapshot.stress_score > self.config.recovery_stress_score {
            return self.blocked(snapshot, context, RecoveryReason::StressStillElevated);
        }

        if !matches!(
            snapshot.liquidity_health,
            LiquidityHealth::Healthy | LiquidityHealth::Watch
        ) {
            return self.blocked(snapshot, context, RecoveryReason::LiquidityNotRecovered);
        }

        if !matches!(
            snapshot.depeg_probability,
            DepegProbability::Low | DepegProbability::Medium
        ) {
            return self.blocked(snapshot, context, RecoveryReason::DepegRiskStillElevated);
        }

        if !matches!(
            snapshot.panic_type,
            PanicType::None | PanicType::WhaleExit | PanicType::BridgeOutflow
        ) {
            return self.blocked(snapshot, context, RecoveryReason::PanicTypeStillActive);
        }

        let cooldown_remaining_slots = self.cooldown_remaining_slots(snapshot, context);
        if cooldown_remaining_slots > 0 {
            return RecoveryDecision {
                can_exit_panic: false,
                next_mode: ProtectionMode::PanicProtection,
                cooldown_remaining_slots,
                reason: RecoveryReason::CooldownRemaining,
            };
        }

        RecoveryDecision {
            can_exit_panic: true,
            next_mode: ProtectionMode::Recovery,
            cooldown_remaining_slots: 0,
            reason: RecoveryReason::ConditionsMet,
        }
    }

    fn blocked(
        &self,
        snapshot: RiskOracleSnapshot,
        context: PolicyContext,
        reason: RecoveryReason,
    ) -> RecoveryDecision {
        RecoveryDecision {
            can_exit_panic: false,
            next_mode: ProtectionMode::PanicProtection,
            cooldown_remaining_slots: self.cooldown_remaining_slots(snapshot, context),
            reason,
        }
    }

    fn cooldown_remaining_slots(
        &self,
        snapshot: RiskOracleSnapshot,
        context: PolicyContext,
    ) -> u64 {
        let elapsed_slots = context
            .current_slot
            .saturating_sub(snapshot.last_updated_slot);
        self.config
            .recovery_cooldown_slots
            .saturating_sub(elapsed_slots)
    }
}

#[cfg(test)]
mod tests {
    use super::{RecoveryConfig, RecoveryEngine};
    use crate::types::{
        DepegProbability, LiquidityHealth, PanicType, PolicyContext, ProtectionMode,
        RecoveryReason, RiskOracleSnapshot,
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
    fn ignores_recovery_when_not_in_emergency_mode() {
        let engine = RecoveryEngine::with_default_config();

        let decision = engine.evaluate(snapshot(), context(ProtectionMode::Normal));

        assert!(!decision.can_exit_panic);
        assert_eq!(decision.next_mode, ProtectionMode::Normal);
        assert_eq!(decision.reason, RecoveryReason::NotInEmergencyMode);
    }

    #[test]
    fn blocks_recovery_when_stress_is_elevated() {
        let engine = RecoveryEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.stress_score = 50;

        let decision = engine.evaluate(snapshot, context(ProtectionMode::PanicProtection));

        assert!(!decision.can_exit_panic);
        assert_eq!(decision.reason, RecoveryReason::StressStillElevated);
    }

    #[test]
    fn blocks_recovery_when_liquidity_has_not_recovered() {
        let engine = RecoveryEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.liquidity_health = LiquidityHealth::Stressed;

        let decision = engine.evaluate(snapshot, context(ProtectionMode::PanicProtection));

        assert!(!decision.can_exit_panic);
        assert_eq!(decision.reason, RecoveryReason::LiquidityNotRecovered);
    }

    #[test]
    fn blocks_recovery_when_depeg_risk_is_high() {
        let engine = RecoveryEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.depeg_probability = DepegProbability::High;

        let decision = engine.evaluate(snapshot, context(ProtectionMode::PanicProtection));

        assert!(!decision.can_exit_panic);
        assert_eq!(decision.reason, RecoveryReason::DepegRiskStillElevated);
    }

    #[test]
    fn blocks_recovery_when_panic_type_is_still_active() {
        let engine = RecoveryEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.panic_type = PanicType::LiquidityPanic;

        let decision = engine.evaluate(snapshot, context(ProtectionMode::PanicProtection));

        assert!(!decision.can_exit_panic);
        assert_eq!(decision.reason, RecoveryReason::PanicTypeStillActive);
    }

    #[test]
    fn reports_remaining_cooldown_slots() {
        let engine = RecoveryEngine::new(RecoveryConfig {
            recovery_stress_score: 45,
            recovery_cooldown_slots: 500,
        });

        let decision = engine.evaluate(snapshot(), context(ProtectionMode::PanicProtection));

        assert!(!decision.can_exit_panic);
        assert_eq!(decision.cooldown_remaining_slots, 300);
        assert_eq!(decision.reason, RecoveryReason::CooldownRemaining);
    }

    #[test]
    fn allows_recovery_when_all_conditions_are_met() {
        let engine = RecoveryEngine::with_default_config();

        let decision = engine.evaluate(snapshot(), context(ProtectionMode::PanicProtection));

        assert!(decision.can_exit_panic);
        assert_eq!(decision.next_mode, ProtectionMode::Recovery);
        assert_eq!(decision.reason, RecoveryReason::ConditionsMet);
    }
}
