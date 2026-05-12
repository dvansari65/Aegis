//! Estimates peg instability risk from price, liquidity, and flow signals.

use crate::signals::MarketSignals;
use crate::types::DepegProbabilityBand;

pub fn calculate_depeg_probability(signals: &MarketSignals) -> DepegProbabilityBand {
    if signals.oracle_price_divergence_bps > 300 || signals.whale_exit_volume > 10_000_000.0 {
        DepegProbabilityBand::VeryHigh
    } else if signals.oracle_price_divergence_bps > 150 || signals.whale_exit_volume > 5_000_000.0 {
        DepegProbabilityBand::High
    } else if signals.oracle_price_divergence_bps > 50 || signals.whale_exit_volume > 1_000_000.0 {
        DepegProbabilityBand::Medium
    } else {
        DepegProbabilityBand::Low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_depeg_probability() {
        let signals = MarketSignals {
            withdrawal_volume_24h: 100_000.0,
            pool_imbalance_ratio: 0.50,
            oracle_price_divergence_bps: 10,
            slippage_bps: 10,
            whale_exit_volume: 500_000.0,
        };
        assert_eq!(calculate_depeg_probability(&signals), DepegProbabilityBand::Low);
    }

    #[test]
    fn test_very_high_divergence() {
        let signals = MarketSignals {
            withdrawal_volume_24h: 100_000.0,
            pool_imbalance_ratio: 0.50,
            oracle_price_divergence_bps: 350, // > 300
            slippage_bps: 10,
            whale_exit_volume: 0.0,
        };
        assert_eq!(calculate_depeg_probability(&signals), DepegProbabilityBand::VeryHigh);
    }

    #[test]
    fn test_high_whale_exit() {
        let signals = MarketSignals {
            withdrawal_volume_24h: 100_000.0,
            pool_imbalance_ratio: 0.50,
            oracle_price_divergence_bps: 0,
            slippage_bps: 10,
            whale_exit_volume: 6_000_000.0, // > 5,000,000
        };
        assert_eq!(calculate_depeg_probability(&signals), DepegProbabilityBand::High);
    }
}
