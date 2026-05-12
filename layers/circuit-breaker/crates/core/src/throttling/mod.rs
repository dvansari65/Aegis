//! Defines withdrawal delay, queue, and rate-limit policies during panic.

use serde::{Deserialize, Serialize};

use crate::types::{
    CircuitBreakerDecision, DepegProbability, LiquidityHealth, ProtectionMode,
    WithdrawalThrottlePolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalThrottleConfig {
    pub normal_max_withdrawal_bps: u16,
    pub watch_max_withdrawal_bps: u16,
    pub recovery_max_withdrawal_bps: u16,
    pub panic_max_withdrawal_bps: u16,
    pub severe_panic_max_withdrawal_bps: u16,
    pub window_slots: u64,
    pub watch_delay_slots: u64,
    pub panic_delay_slots: u64,
    pub severe_panic_delay_slots: u64,
    pub recovery_delay_slots: u64,
    pub large_withdrawal_threshold_bps: u16,
}

impl Default for WithdrawalThrottleConfig {
    fn default() -> Self {
        Self {
            normal_max_withdrawal_bps: 10_000,
            watch_max_withdrawal_bps: 5_000,
            recovery_max_withdrawal_bps: 3_000,
            panic_max_withdrawal_bps: 1_000,
            severe_panic_max_withdrawal_bps: 500,
            window_slots: 150,
            watch_delay_slots: 0,
            panic_delay_slots: 75,
            severe_panic_delay_slots: 150,
            recovery_delay_slots: 25,
            large_withdrawal_threshold_bps: 250,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WithdrawalThrottleEngine {
    config: WithdrawalThrottleConfig,
}

impl WithdrawalThrottleEngine {
    pub fn new(config: WithdrawalThrottleConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(WithdrawalThrottleConfig::default())
    }

    pub fn compute(&self, decision: &CircuitBreakerDecision) -> WithdrawalThrottlePolicy {
        match decision.mode {
            ProtectionMode::Normal => WithdrawalThrottlePolicy {
                mode: decision.mode,
                max_withdrawal_bps_per_window: self.config.normal_max_withdrawal_bps,
                window_slots: self.config.window_slots,
                delay_slots: 0,
                queue_large_withdrawals: false,
                large_withdrawal_threshold_bps: self.config.large_withdrawal_threshold_bps,
            },
            ProtectionMode::Watch => WithdrawalThrottlePolicy {
                mode: decision.mode,
                max_withdrawal_bps_per_window: self.config.watch_max_withdrawal_bps,
                window_slots: self.config.window_slots,
                delay_slots: self.config.watch_delay_slots,
                queue_large_withdrawals: false,
                large_withdrawal_threshold_bps: self.config.large_withdrawal_threshold_bps,
            },
            ProtectionMode::Recovery => WithdrawalThrottlePolicy {
                mode: decision.mode,
                max_withdrawal_bps_per_window: self.config.recovery_max_withdrawal_bps,
                window_slots: self.config.window_slots,
                delay_slots: self.config.recovery_delay_slots,
                queue_large_withdrawals: true,
                large_withdrawal_threshold_bps: self.config.large_withdrawal_threshold_bps,
            },
            ProtectionMode::PanicProtection => self.panic_policy(decision),
        }
    }

    fn panic_policy(&self, decision: &CircuitBreakerDecision) -> WithdrawalThrottlePolicy {
        let severe = is_severe_panic(decision);

        WithdrawalThrottlePolicy {
            mode: decision.mode,
            max_withdrawal_bps_per_window: if severe {
                self.config.severe_panic_max_withdrawal_bps
            } else {
                self.config.panic_max_withdrawal_bps
            },
            window_slots: self.config.window_slots,
            delay_slots: if severe {
                self.config.severe_panic_delay_slots
            } else {
                self.config.panic_delay_slots
            },
            queue_large_withdrawals: true,
            large_withdrawal_threshold_bps: self.config.large_withdrawal_threshold_bps,
        }
    }
}

fn is_severe_panic(decision: &CircuitBreakerDecision) -> bool {
    matches!(decision.snapshot.liquidity_health, LiquidityHealth::Severe)
        || matches!(
            decision.snapshot.depeg_probability,
            DepegProbability::VeryHigh
        )
        || decision.snapshot.stress_score >= 95
}

#[cfg(test)]
mod tests {
    use super::WithdrawalThrottleEngine;
    use crate::{
        policy::CircuitBreakerPolicyEngine,
        types::{
            DepegProbability, LiquidityHealth, PanicType, PolicyContext, ProtectionMode,
            RiskOracleSnapshot,
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
    fn disables_throttling_in_normal_mode() {
        let throttling = WithdrawalThrottleEngine::with_default_config();

        let policy = throttling.compute(&decision(snapshot(), ProtectionMode::Normal));

        assert_eq!(policy.mode, ProtectionMode::Normal);
        assert_eq!(policy.max_withdrawal_bps_per_window, 10_000);
        assert_eq!(policy.delay_slots, 0);
        assert!(!policy.queue_large_withdrawals);
    }

    #[test]
    fn applies_soft_limit_in_watch_mode() {
        let throttling = WithdrawalThrottleEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.stress_score = 60;

        let policy = throttling.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.mode, ProtectionMode::Watch);
        assert_eq!(policy.max_withdrawal_bps_per_window, 5_000);
        assert!(!policy.queue_large_withdrawals);
    }

    #[test]
    fn queues_large_withdrawals_in_panic_mode() {
        let throttling = WithdrawalThrottleEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.stress_score = 90;

        let policy = throttling.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.mode, ProtectionMode::PanicProtection);
        assert_eq!(policy.max_withdrawal_bps_per_window, 1_000);
        assert_eq!(policy.delay_slots, 75);
        assert!(policy.queue_large_withdrawals);
    }

    #[test]
    fn tightens_limits_for_severe_panic() {
        let throttling = WithdrawalThrottleEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.liquidity_health = LiquidityHealth::Severe;

        let policy = throttling.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.max_withdrawal_bps_per_window, 500);
        assert_eq!(policy.delay_slots, 150);
        assert!(policy.queue_large_withdrawals);
    }

    #[test]
    fn keeps_mild_throttle_during_recovery() {
        let throttling = WithdrawalThrottleEngine::with_default_config();

        let policy = throttling.compute(&decision(snapshot(), ProtectionMode::PanicProtection));

        assert_eq!(policy.mode, ProtectionMode::Recovery);
        assert_eq!(policy.max_withdrawal_bps_per_window, 3_000);
        assert_eq!(policy.delay_slots, 25);
        assert!(policy.queue_large_withdrawals);
    }
}
