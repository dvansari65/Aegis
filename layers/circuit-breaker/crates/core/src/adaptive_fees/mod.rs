//! Maps panic severity into temporary fee policies.

use serde::{Deserialize, Serialize};

use crate::types::{
    AdaptiveFeePolicy, CircuitBreakerDecision, DepegProbability, LiquidityHealth, ProtectionMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptiveFeeConfig {
    pub normal_fee_bps: u16,
    pub watch_fee_bps: u16,
    pub recovery_fee_bps: u16,
    pub panic_min_fee_bps: u16,
    pub panic_max_fee_bps: u16,
}

impl Default for AdaptiveFeeConfig {
    fn default() -> Self {
        Self {
            normal_fee_bps: 5,
            watch_fee_bps: 30,
            recovery_fee_bps: 50,
            panic_min_fee_bps: 200,
            panic_max_fee_bps: 500,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AdaptiveFeeEngine {
    config: AdaptiveFeeConfig,
}

impl AdaptiveFeeEngine {
    pub fn new(config: AdaptiveFeeConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(AdaptiveFeeConfig::default())
    }

    pub fn compute(&self, decision: &CircuitBreakerDecision) -> AdaptiveFeePolicy {
        let fee_bps = match decision.mode {
            ProtectionMode::Normal => self.config.normal_fee_bps,
            ProtectionMode::Watch => self.config.watch_fee_bps,
            ProtectionMode::Recovery => self.config.recovery_fee_bps,
            ProtectionMode::PanicProtection => self.panic_fee_bps(decision),
        };

        AdaptiveFeePolicy {
            fee_bps,
            max_fee_bps: self.config.panic_max_fee_bps,
            mode: decision.mode,
        }
    }

    fn panic_fee_bps(&self, decision: &CircuitBreakerDecision) -> u16 {
        let stress_fee = interpolate(
            decision.snapshot.stress_score,
            85,
            100,
            self.config.panic_min_fee_bps,
            self.config.panic_max_fee_bps,
        );

        let liquidity_fee = match decision.snapshot.liquidity_health {
            LiquidityHealth::Healthy | LiquidityHealth::Watch => self.config.panic_min_fee_bps,
            LiquidityHealth::Stressed => 250,
            LiquidityHealth::Critical => 350,
            LiquidityHealth::Severe => self.config.panic_max_fee_bps,
        };

        let depeg_fee = match decision.snapshot.depeg_probability {
            DepegProbability::Low | DepegProbability::Medium => self.config.panic_min_fee_bps,
            DepegProbability::High => 350,
            DepegProbability::VeryHigh => self.config.panic_max_fee_bps,
        };

        stress_fee
            .max(liquidity_fee)
            .max(depeg_fee)
            .clamp(self.config.panic_min_fee_bps, self.config.panic_max_fee_bps)
    }
}

fn interpolate(value: u8, start: u8, end: u8, min: u16, max: u16) -> u16 {
    if value <= start {
        return min;
    }

    if value >= end {
        return max;
    }

    let span = u16::from(end - start);
    let offset = u16::from(value - start);
    let range = max - min;

    min + (range * offset / span)
}

#[cfg(test)]
mod tests {
    use super::AdaptiveFeeEngine;
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
    fn returns_normal_fee_in_normal_mode() {
        let fees = AdaptiveFeeEngine::with_default_config();
        let policy = fees.compute(&decision(snapshot(), ProtectionMode::Normal));

        assert_eq!(policy.fee_bps, 5);
        assert_eq!(policy.mode, ProtectionMode::Normal);
    }

    #[test]
    fn returns_watch_fee_in_watch_mode() {
        let fees = AdaptiveFeeEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.stress_score = 60;

        let policy = fees.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.fee_bps, 30);
        assert_eq!(policy.mode, ProtectionMode::Watch);
    }

    #[test]
    fn scales_panic_fee_with_stress_score() {
        let fees = AdaptiveFeeEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.stress_score = 93;

        let policy = fees.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.fee_bps, 360);
        assert_eq!(policy.mode, ProtectionMode::PanicProtection);
    }

    #[test]
    fn caps_fee_during_severe_liquidity_stress() {
        let fees = AdaptiveFeeEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.liquidity_health = LiquidityHealth::Severe;

        let policy = fees.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.fee_bps, 500);
    }

    #[test]
    fn keeps_recovery_fee_elevated_but_below_panic() {
        let fees = AdaptiveFeeEngine::with_default_config();
        let policy = fees.compute(&decision(snapshot(), ProtectionMode::PanicProtection));

        assert_eq!(policy.fee_bps, 50);
        assert_eq!(policy.mode, ProtectionMode::Recovery);
    }
}
