//! Combines individual signals into stablecoin-level stress scores.

use crate::depeg::calculate_depeg_probability;
use crate::liquidity::calculate_liquidity_health;
use crate::signals::MarketSignals;
use crate::types::{DepegProbabilityBand, LiquidityHealth};

pub fn calculate_stress_score(signals: &MarketSignals) -> u8 {
    let mut score: u8 = 0;

    let liquidity_health = calculate_liquidity_health(signals);
    let depeg_prob = calculate_depeg_probability(signals);

    // Baseline from liquidity health (max 40 points)
    score += match liquidity_health {
        LiquidityHealth::Healthy => 0,
        LiquidityHealth::Watch => 10,
        LiquidityHealth::Stressed => 20,
        LiquidityHealth::Critical => 30,
        LiquidityHealth::Severe => 40,
    };

    // Baseline from depeg probability (max 40 points)
    score += match depeg_prob {
        DepegProbabilityBand::Low => 0,
        DepegProbabilityBand::Medium => 10,
        DepegProbabilityBand::High => 25,
        DepegProbabilityBand::VeryHigh => 40,
    };

    // Add immediate panic indicators (max 20 points)
    if signals.oracle_price_divergence_bps > 100 {
        score += 10;
    }
    
    if signals.pool_imbalance_ratio > 0.75 {
        score += 10;
    }

    // Cap at 100
    if score > 100 {
        100
    } else {
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_health_score() {
        let signals = MarketSignals {
            withdrawal_volume_24h: 0.0,
            pool_imbalance_ratio: 0.50,
            oracle_price_divergence_bps: 0,
            slippage_bps: 10,
            whale_exit_volume: 0.0,
        };
        // Healthy liquidity (0) + Low depeg (0) = 0
        assert_eq!(calculate_stress_score(&signals), 0);
    }

    #[test]
    fn test_max_panic_score() {
        let signals = MarketSignals {
            withdrawal_volume_24h: 100_000_000.0,
            pool_imbalance_ratio: 0.90, // Severe liquidity (+40), Panic Imbalance (+10)
            oracle_price_divergence_bps: 400, // VeryHigh depeg (+40), Panic Divergence (+10)
            slippage_bps: 600,
            whale_exit_volume: 20_000_000.0,
        };
        // 40 + 40 + 10 + 10 = 100
        assert_eq!(calculate_stress_score(&signals), 100);
    }

    #[test]
    fn test_medium_stress_score() {
        let signals = MarketSignals {
            withdrawal_volume_24h: 5_000_000.0,
            pool_imbalance_ratio: 0.65, // Stressed liquidity (+20)
            oracle_price_divergence_bps: 60, // Medium depeg (+10)
            slippage_bps: 50,
            whale_exit_volume: 0.0,
        };
        // 20 + 10 = 30
        assert_eq!(calculate_stress_score(&signals), 30);
    }
}
