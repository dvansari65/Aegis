//! Chooses liquidity routing and rebalancing policies under stress.

use serde::{Deserialize, Serialize};

use crate::types::{
    CircuitBreakerDecision, DepegProbability, LiquidityHealth, LiquidityRoutingPolicy, PanicType,
    ProtectionMode, RoutingMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiquidityRoutingConfig {
    pub normal_min_liquidity_depth_bps: u16,
    pub watch_min_liquidity_depth_bps: u16,
    pub panic_min_liquidity_depth_bps: u16,
    pub emergency_min_liquidity_depth_bps: u16,
    pub normal_max_single_route_share_bps: u16,
    pub split_max_single_route_share_bps: u16,
    pub emergency_max_single_route_share_bps: u16,
}

impl Default for LiquidityRoutingConfig {
    fn default() -> Self {
        Self {
            normal_min_liquidity_depth_bps: 100,
            watch_min_liquidity_depth_bps: 250,
            panic_min_liquidity_depth_bps: 500,
            emergency_min_liquidity_depth_bps: 1_000,
            normal_max_single_route_share_bps: 10_000,
            split_max_single_route_share_bps: 4_000,
            emergency_max_single_route_share_bps: 2_500,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LiquidityRoutingEngine {
    config: LiquidityRoutingConfig,
}

impl LiquidityRoutingEngine {
    pub fn new(config: LiquidityRoutingConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(LiquidityRoutingConfig::default())
    }

    pub fn compute(&self, decision: &CircuitBreakerDecision) -> LiquidityRoutingPolicy {
        match decision.mode {
            ProtectionMode::Normal => LiquidityRoutingPolicy {
                mode: decision.mode,
                routing_mode: RoutingMode::Normal,
                min_liquidity_depth_bps: self.config.normal_min_liquidity_depth_bps,
                max_single_route_share_bps: self.config.normal_max_single_route_share_bps,
                allow_stressed_pools: true,
                require_oracle_consistency: false,
            },
            ProtectionMode::Watch => LiquidityRoutingPolicy {
                mode: decision.mode,
                routing_mode: RoutingMode::PreferDeepLiquidity,
                min_liquidity_depth_bps: self.config.watch_min_liquidity_depth_bps,
                max_single_route_share_bps: self.config.normal_max_single_route_share_bps,
                allow_stressed_pools: true,
                require_oracle_consistency: true,
            },
            ProtectionMode::Recovery => LiquidityRoutingPolicy {
                mode: decision.mode,
                routing_mode: RoutingMode::SplitAcrossDeepRoutes,
                min_liquidity_depth_bps: self.config.panic_min_liquidity_depth_bps,
                max_single_route_share_bps: self.config.split_max_single_route_share_bps,
                allow_stressed_pools: false,
                require_oracle_consistency: true,
            },
            ProtectionMode::PanicProtection => self.panic_policy(decision),
        }
    }

    fn panic_policy(&self, decision: &CircuitBreakerDecision) -> LiquidityRoutingPolicy {
        let emergency = requires_emergency_routing(decision);

        LiquidityRoutingPolicy {
            mode: decision.mode,
            routing_mode: if emergency {
                RoutingMode::EmergencyOnly
            } else {
                RoutingMode::AvoidStressedPools
            },
            min_liquidity_depth_bps: if emergency {
                self.config.emergency_min_liquidity_depth_bps
            } else {
                self.config.panic_min_liquidity_depth_bps
            },
            max_single_route_share_bps: if emergency {
                self.config.emergency_max_single_route_share_bps
            } else {
                self.config.split_max_single_route_share_bps
            },
            allow_stressed_pools: false,
            require_oracle_consistency: true,
        }
    }
}

fn requires_emergency_routing(decision: &CircuitBreakerDecision) -> bool {
    matches!(decision.snapshot.liquidity_health, LiquidityHealth::Severe)
        || matches!(
            decision.snapshot.depeg_probability,
            DepegProbability::VeryHigh
        )
        || matches!(
            decision.snapshot.panic_type,
            PanicType::OracleDislocation | PanicType::PossibleManipulation
        )
        || decision.snapshot.stress_score >= 95
}

#[cfg(test)]
mod tests {
    use super::LiquidityRoutingEngine;
    use crate::{
        policy::CircuitBreakerPolicyEngine,
        types::{
            DepegProbability, LiquidityHealth, PanicType, PolicyContext, ProtectionMode,
            RiskOracleSnapshot, RoutingMode,
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
    fn allows_normal_routing_in_normal_mode() {
        let routing = LiquidityRoutingEngine::with_default_config();

        let policy = routing.compute(&decision(snapshot(), ProtectionMode::Normal));

        assert_eq!(policy.routing_mode, RoutingMode::Normal);
        assert!(policy.allow_stressed_pools);
        assert!(!policy.require_oracle_consistency);
    }

    #[test]
    fn prefers_deep_liquidity_in_watch_mode() {
        let routing = LiquidityRoutingEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.stress_score = 60;

        let policy = routing.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.routing_mode, RoutingMode::PreferDeepLiquidity);
        assert!(policy.allow_stressed_pools);
        assert!(policy.require_oracle_consistency);
    }

    #[test]
    fn avoids_stressed_pools_in_panic_mode() {
        let routing = LiquidityRoutingEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.stress_score = 90;

        let policy = routing.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.routing_mode, RoutingMode::AvoidStressedPools);
        assert_eq!(policy.max_single_route_share_bps, 4_000);
        assert!(!policy.allow_stressed_pools);
    }

    #[test]
    fn uses_emergency_routing_for_oracle_dislocation() {
        let routing = LiquidityRoutingEngine::with_default_config();
        let mut snapshot = snapshot();
        snapshot.panic_type = PanicType::OracleDislocation;

        let policy = routing.compute(&decision(snapshot, ProtectionMode::Normal));

        assert_eq!(policy.routing_mode, RoutingMode::EmergencyOnly);
        assert_eq!(policy.min_liquidity_depth_bps, 1_000);
        assert_eq!(policy.max_single_route_share_bps, 2_500);
    }

    #[test]
    fn keeps_split_routing_during_recovery() {
        let routing = LiquidityRoutingEngine::with_default_config();

        let policy = routing.compute(&decision(snapshot(), ProtectionMode::PanicProtection));

        assert_eq!(policy.mode, ProtectionMode::Recovery);
        assert_eq!(policy.routing_mode, RoutingMode::SplitAcrossDeepRoutes);
        assert!(!policy.allow_stressed_pools);
    }
}
