//! Classifies pool depth, imbalance, and slippage risk into liquidity health.

use crate::signals::MarketSignals;
use crate::types::LiquidityHealth;

pub fn calculate_liquidity_health(signals: &MarketSignals) -> LiquidityHealth {
    if signals.pool_imbalance_ratio > 0.85 || signals.slippage_bps > 500 {
        LiquidityHealth::Severe
    } else if signals.pool_imbalance_ratio > 0.70 || signals.slippage_bps > 200 {
        LiquidityHealth::Critical
    } else if signals.pool_imbalance_ratio > 0.60 || signals.slippage_bps > 100 {
        LiquidityHealth::Stressed
    } else if signals.pool_imbalance_ratio > 0.55 || signals.slippage_bps > 50 {
        LiquidityHealth::Watch
    } else {
        LiquidityHealth::Healthy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy_liquidity() {
        let signals = MarketSignals {
            withdrawal_volume_24h: 100_000.0,
            pool_imbalance_ratio: 0.50,
            oracle_price_divergence_bps: 0,
            slippage_bps: 10,
            whale_exit_volume: 0.0,
        };
        assert_eq!(calculate_liquidity_health(&signals), LiquidityHealth::Healthy);
    }

    #[test]
    fn test_severe_imbalance() {
        let signals = MarketSignals {
            withdrawal_volume_24h: 100_000.0,
            pool_imbalance_ratio: 0.86,
            oracle_price_divergence_bps: 0,
            slippage_bps: 10,
            whale_exit_volume: 0.0,
        };
        assert_eq!(calculate_liquidity_health(&signals), LiquidityHealth::Severe);
    }

    #[test]
    fn test_stressed_slippage() {
        let signals = MarketSignals {
            withdrawal_volume_24h: 100_000.0,
            pool_imbalance_ratio: 0.50,
            oracle_price_divergence_bps: 0,
            slippage_bps: 150,
            whale_exit_volume: 0.0,
        };
        assert_eq!(calculate_liquidity_health(&signals), LiquidityHealth::Stressed);
    }
}
