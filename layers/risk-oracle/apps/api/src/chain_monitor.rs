//! Optional Solana JSON-RPC monitor: read risk oracle state + (optional) CB mode, then
//! run the same policy engines as the keeper for the dashboard.

use base64::Engine;
use reqwest::Client;
use serde_json::{Value, json};
use shock_absorber_circuit_breaker::risk_oracle_feed::risk_state_to_snapshot;
use shock_absorber_circuit_breaker::types::ProtectionMode;

use crate::config::ChainMonitorConfig;
use crate::http::monitor::{
    now_ms, CircuitBreakerView, CoreMetrics, MonitorResponse, SecondarySignals,
};
use crate::policy_wire::{self, DerivedCbView};

const CB_STATE_LEN: usize = 45;

async fn rpc_post(client: &Client, rpc_url: &str, body: Value) -> Result<Value, String> {
    let response = client
        .post(rpc_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("rpc http: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("rpc status {}", response.status()));
    }

    response
        .json::<Value>()
        .await
        .map_err(|e| format!("rpc json: {e}"))
}

pub async fn fetch_slot(client: &Client, rpc_url: &str) -> Result<u64, String> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1u64,
        "method": "getSlot",
        "params": [{"commitment": "processed"}]
    });
    let v = rpc_post(client, rpc_url, body).await?;
    v.get("result")
        .and_then(|r| r.as_u64())
        .ok_or_else(|| "getSlot: missing result".to_owned())
}

async fn fetch_account_base64(client: &Client, rpc_url: &str, pubkey: &str) -> Result<Vec<u8>, String> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1u64,
        "method": "getAccountInfo",
        "params": [pubkey, {"encoding": "base64", "commitment": "processed"}]
    });
    let v = rpc_post(client, rpc_url, body).await?;
    let value = v
        .pointer("/result/value")
        .ok_or_else(|| "getAccountInfo: missing result.value".to_owned())?;
    if value.is_null() {
        return Err(format!("getAccountInfo: account not found ({pubkey})"));
    }
    let arr = value
        .get("data")
        .and_then(|d| d.as_array())
        .ok_or_else(|| "getAccountInfo: missing data array".to_owned())?;
    let b64 = arr
        .first()
        .and_then(|x| x.as_str())
        .ok_or_else(|| "getAccountInfo: missing base64 string".to_owned())?;
    base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| format!("base64: {e}"))
}

fn decode_cb_mode(data: &[u8]) -> ProtectionMode {
    if data.len() < CB_STATE_LEN {
        return ProtectionMode::Normal;
    }
    ProtectionMode::try_from_discriminant(data[32]).unwrap_or(ProtectionMode::Normal)
}

async fn fetch_previous_mode(
    client: &Client,
    rpc_url: &str,
    cb_pubkey: Option<&String>,
) -> Result<ProtectionMode, String> {
    let Some(pk) = cb_pubkey else {
        return Ok(ProtectionMode::Normal);
    };
    let data = fetch_account_base64(client, rpc_url, pk).await;
    match data {
        Ok(bytes) => Ok(decode_cb_mode(&bytes)),
        Err(e) => {
            tracing::warn!(%e, "circuit breaker account read failed; assuming Normal");
            Ok(ProtectionMode::Normal)
        }
    }
}

fn cb_view_to_wire(v: DerivedCbView) -> CircuitBreakerView {
    CircuitBreakerView {
        protection_mode: v.protection_mode,
        dynamic_fees_enabled: v.dynamic_fees_enabled,
        withdrawal_throttling_active: v.withdrawal_throttling_active,
        liquidity_routing_active: v.liquidity_routing_active,
        toxic_routing_restricted: v.toxic_routing_restricted,
    }
}

fn secondary_from_chain(slot: u64, last_oracle_slot: u64) -> SecondarySignals {
    SecondarySignals {
        withdrawal_velocity: format!("Chain slot {slot} (risk oracle slot {last_oracle_slot})"),
        pool_imbalance: "Not stored in risk state account (use ingestion / DEX sources)".to_owned(),
        whale_abnormal_exits: 0,
        oracle_divergence_bps: 0,
        bridge_outflows: "Not stored in risk state account (use bridge indexers)".to_owned(),
    }
}

pub async fn fetch_chain_monitor(
    client: &Client,
    cfg: &ChainMonitorConfig,
    symbol: &str,
) -> Result<MonitorResponse, String> {
    let slot = fetch_slot(client, &cfg.rpc_url).await?;
    let risk_bytes = fetch_account_base64(client, &cfg.rpc_url, &cfg.risk_pubkey).await?;
    let snapshot = risk_state_to_snapshot(&risk_bytes, cfg.oracle_confidence, slot)
        .map_err(|e| format!("risk oracle account: {e}"))?;

    let previous_mode = fetch_previous_mode(client, &cfg.rpc_url, cfg.cb_pubkey.as_ref()).await?;

    let cb_derived = policy_wire::derive_cb_view(slot, snapshot, previous_mode);

    let sym = symbol.trim().to_uppercase();
    let now = now_ms();

    Ok(MonitorResponse {
        symbol: if sym.is_empty() {
            "UNKNOWN".to_owned()
        } else {
            sym
        },
        updated_at_unix_ms: now,
        data_source: "solana".to_owned(),
        core: CoreMetrics {
            stress_score: snapshot.stress_score,
            stress_max: 100,
            liquidity_health: policy_wire::liquidity_health_label(snapshot.liquidity_health),
            depeg_probability: policy_wire::depeg_probability_label(snapshot.depeg_probability),
        },
        circuit_breaker: cb_view_to_wire(cb_derived),
        secondary: secondary_from_chain(slot, snapshot.last_updated_slot),
    })
}
