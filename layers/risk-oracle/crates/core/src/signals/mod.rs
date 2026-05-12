//! Computes normalized panic signals from current and historical market state.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSignals {
    /// 24-hour withdrawal volume across monitored pools
    pub withdrawal_volume_24h: f64,
    /// The current imbalance ratio of the primary liquidity pool (0.0 to 1.0)
    pub pool_imbalance_ratio: f64,
    /// Absolute divergence of the current spot price from the oracle EMA price (in basis points)
    pub oracle_price_divergence_bps: u16,
    /// Current average slippage for a standard size swap (in basis points)
    pub slippage_bps: u16,
    /// Total volume of whale exits detected in the last window
    pub whale_exit_volume: f64,
}
