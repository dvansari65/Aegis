//! JSON contract for the operator dashboard (`apps/frontend` monitor view).
//!
//! - **`demo-live`**: wall-clock–driven synthetic series so values change between polls (local dev).
//! - **`solana`**: filled by `chain_monitor` when RPC + pubkey env is configured.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MonitorResponse {
    pub symbol: String,
    pub updated_at_unix_ms: i64,
    /// `demo-live` = synthetic clock-synced feed; `solana` = JSON-RPC risk account + policy engines.
    pub data_source: String,
    pub core: CoreMetrics,
    pub circuit_breaker: CircuitBreakerView,
    pub secondary: SecondarySignals,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoreMetrics {
    /// 0–100 stress score (headline metric).
    pub stress_score: u8,
    pub stress_max: u8,
    /// One of: `Healthy`, `Watch`, `Warning`, `Stressed`, `Critical`, `Severe`
    pub liquidity_health: String,
    /// One of: `LOW`, `MEDIUM`, `HIGH`, `VERY_HIGH`
    pub depeg_probability: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CircuitBreakerView {
    /// One of: `NORMAL`, `WATCH`, `PANIC PROTECTION`, `RECOVERY`
    pub protection_mode: String,
    pub dynamic_fees_enabled: bool,
    pub withdrawal_throttling_active: bool,
    pub liquidity_routing_active: bool,
    pub toxic_routing_restricted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SecondarySignals {
    pub withdrawal_velocity: String,
    pub pool_imbalance: String,
    pub whale_abnormal_exits: u32,
    pub oracle_divergence_bps: u32,
    pub bridge_outflows: String,
}

pub fn build_monitor(symbol: &str) -> MonitorResponse {
    let t = now_ms();
    let sym = symbol.trim().to_uppercase();
    let stressed = matches!(sym.as_str(), "USDC" | "USDT" | "PYUSD");

    if stressed {
        demo_stressed(sym, t)
    } else {
        demo_calm(sym, t)
    }
}

fn phase(t: i64, period_ms: i64, span: i64) -> i64 {
    if period_ms <= 0 || span <= 0 {
        return 0;
    }
    (t / period_ms).rem_euclid(span)
}

fn demo_stressed(symbol: String, t: i64) -> MonitorResponse {
    // Slow wave so each 2s poll catches visible movement.
    let w = phase(t, 1_200, 14) as u8;
    let stress_score = 82u8.saturating_add(w).min(95);
    let sec = (t / 1000).rem_euclid(86_400);

    let liquidity_health = if stress_score >= 90 {
        "Critical"
    } else if stress_score >= 85 {
        "Stressed"
    } else {
        "Warning"
    }
    .to_owned();

    let depeg_probability = if stress_score >= 90 {
        "HIGH"
    } else if stress_score >= 84 {
        "MEDIUM"
    } else {
        "MEDIUM"
    }
    .to_owned();

    let (protection_mode, dynamic_fees_enabled, withdrawal_throttling_active, liquidity_routing_active, toxic_routing_restricted) =
        if stress_score >= 88 {
            (
                "PANIC PROTECTION",
                true,
                true,
                true,
                true,
            )
        } else if stress_score >= 80 {
            ("WATCH", true, false, true, false)
        } else {
            ("WATCH", false, false, true, false)
        };

    let whale = 9u32.saturating_add(phase(t, 2_500, 6) as u32);
    let bps = 34u32.saturating_add(phase(t, 1_800, 28) as u32);
    let flow_amp = 115u32.saturating_add(phase(t, 3_400, 25) as u32);

    MonitorResponse {
        symbol,
        updated_at_unix_ms: t,
        data_source: "demo-live".to_owned(),
        core: CoreMetrics {
            stress_score,
            stress_max: 100,
            liquidity_health,
            depeg_probability,
        },
        circuit_breaker: CircuitBreakerView {
            protection_mode: protection_mode.to_owned(),
            dynamic_fees_enabled,
            withdrawal_throttling_active,
            liquidity_routing_active,
            toxic_routing_restricted,
        },
        secondary: SecondarySignals {
            withdrawal_velocity: format!(
                "${}M–${}M / 10 min · tick {sec}s (server clock)",
                468 + (w as i64 % 10),
                478 + (w as i64 % 14)
            ),
            pool_imbalance: format!(
                "USDC / SOL → {}% imbalance · rolling",
                78 + (phase(t, 2_200, 15) as i64)
            ),
            whale_abnormal_exits: whale,
            oracle_divergence_bps: bps,
            bridge_outflows: format!(
                "${}M bridged out · rolling window",
                flow_amp
            ),
        },
    }
}

fn demo_calm(symbol: String, t: i64) -> MonitorResponse {
    let w = phase(t, 1_800, 7) as u8;
    let stress_score = 22u8.saturating_add(w).min(34);
    let sec = (t / 1000).rem_euclid(86_400);

    MonitorResponse {
        symbol,
        updated_at_unix_ms: t,
        data_source: "demo-live".to_owned(),
        core: CoreMetrics {
            stress_score,
            stress_max: 100,
            liquidity_health: "Healthy".to_owned(),
            depeg_probability: "LOW".to_owned(),
        },
        circuit_breaker: CircuitBreakerView {
            protection_mode: "NORMAL".to_owned(),
            dynamic_fees_enabled: false,
            withdrawal_throttling_active: false,
            liquidity_routing_active: true,
            toxic_routing_restricted: false,
        },
        secondary: SecondarySignals {
            withdrawal_velocity: format!(
                "${}M–${}M / 10 min · tick {sec}s (server clock)",
                38 + (w as i64 % 6),
                46 + (w as i64 % 8)
            ),
            pool_imbalance: format!(
                "USDC / SOL → {}% imbalance",
                8 + (phase(t, 2_600, 8) as i64)
            ),
            whale_abnormal_exits: 0,
            oracle_divergence_bps: 4 + phase(t, 2_100, 8) as u32,
            bridge_outflows: format!(
                "${}M bridged out · tick {sec}s",
                5 + phase(t, 3_100, 6) as u32
            ),
        },
    }
}

pub fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
