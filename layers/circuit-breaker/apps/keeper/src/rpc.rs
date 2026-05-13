use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json};

use crate::error::KeeperError;

fn post(rpc_url: &str, body: Value) -> Result<Value, KeeperError> {
    ureq::post(rpc_url)
        .send_json(body)
        .map_err(|e| KeeperError::Rpc(e.to_string()))?
        .into_json::<Value>()
        .map_err(|e| KeeperError::Rpc(e.to_string()))
}

pub fn get_slot(rpc_url: &str) -> Result<u64, KeeperError> {
    let v = post(
        rpc_url,
        json!({
            "jsonrpc": "2.0",
            "id": 1u64,
            "method": "getSlot",
            "params": [{ "commitment": "processed" }]
        }),
    )?;

    v.get("result")
        .and_then(|r| r.as_u64())
        .ok_or_else(|| KeeperError::Rpc("getSlot: missing result".to_owned()))
}

pub fn get_account_data(rpc_url: &str, pubkey_b58: &str) -> Result<Vec<u8>, KeeperError> {
    let v = post(
        rpc_url,
        json!({
            "jsonrpc": "2.0",
            "id": 1u64,
            "method": "getAccountInfo",
            "params": [
                pubkey_b58,
                { "encoding": "base64" }
            ]
        }),
    )?;

    let value = v
        .get("result")
        .and_then(|r| r.get("value"))
        .ok_or_else(|| KeeperError::Rpc("getAccountInfo: missing result.value".to_owned()))?;

    if value.is_null() {
        return Err(KeeperError::Rpc(format!(
            "account {pubkey_b58} not found (null value)"
        )));
    }

    let data = value
        .get("data")
        .and_then(|d| d.as_array())
        .and_then(|arr| arr.first())
        .and_then(|x| x.as_str())
        .ok_or_else(|| KeeperError::Rpc("getAccountInfo: missing data[0] base64".to_owned()))?;

    STANDARD
        .decode(data)
        .map_err(|e| KeeperError::Rpc(format!("base64 decode: {e}")))
}
