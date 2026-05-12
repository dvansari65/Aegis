use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    // Every raw source is converted into one shared envelope so downstream
    // scoring, storage, and alerting can work with a single event shape.
    pub observed_at_unix_ms: u128,
    pub source: EventSource,
    pub payload: EventPayload,
}

impl EventEnvelope {
    pub fn new(source: EventSource, payload: EventPayload, observed_at: SystemTime) -> Self {
        let observed_at_unix_ms = observed_at
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        Self {
            observed_at_unix_ms,
            source,
            payload,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventSource {
    SolanaRpc,
    PythHermes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EventPayload {
    // Slot snapshots provide a lightweight heartbeat and help tie account reads
    // back to chain progress.
    RpcSlot {
        slot: u64,
    },
    // Account snapshots are intentionally generic for now; later stages can
    // decode DEX- or vault-specific account layouts on top of this baseline.
    AccountSnapshot {
        pubkey: String,
        slot: u64,
        lamports: u64,
        owner: String,
        executable: bool,
        rent_epoch: u64,
        data_len: usize,
    },
    // Oracle snapshots are normalized into integer fields so the scoring layer
    // can reason about prices without depending on source-specific JSON.
    OraclePriceSnapshot {
        feed_id: String,
        price: i64,
        confidence: u64,
        exponent: i32,
        publish_time: i64,
        ema_price: i64,
        ema_confidence: u64,
    },
    // Large stablecoin transfers detected on-chain, indicative of potential
    // liquidity exits or panic behavior.
    WhaleTransfer {
        signature: String,
        mint: String,
        amount: u64,
        from: String,
        to: String,
        slot: u64,
    },
}
