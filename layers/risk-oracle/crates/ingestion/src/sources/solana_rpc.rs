use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::time::SystemTime;
use url::Url;

use crate::{
    error::SourceError,
    models::{EventEnvelope, EventPayload, EventSource},
    sources::Source,
};

pub struct SolanaRpcSource {
    client: Client,
    rpc_url: Url,
    monitored_accounts: Vec<String>,
}

impl SolanaRpcSource {
    pub fn new(rpc_url: Url, monitored_accounts: Vec<String>) -> Self {
        Self {
            client: Client::new(),
            rpc_url,
            monitored_accounts,
        }
    }

    async fn fetch_slot(&self) -> Result<u64, SourceError> {
        // Slot polling gives the service a simple chain-progress signal without
        // requiring any program-specific decoding.
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1u64,
            "method": "getSlot",
            "params": [{"commitment": "confirmed"}],
        });

        let response = self
            .client
            .post(self.rpc_url.clone())
            .json(&payload)
            .send()
            .await?;
        let body: RpcEnvelope<u64> = response.error_for_status()?.json().await?;

        body.result
            .ok_or_else(|| SourceError::MalformedResponse("missing slot result".to_owned()))
    }

    async fn fetch_accounts(&self) -> Result<RpcAccountsResult, SourceError> {
        // We request only metadata here. Full binary account decoding belongs in
        // later protocol-specific parsers once the monitored account set stabilizes.
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 2u64,
            "method": "getMultipleAccounts",
            "params": [
                self.monitored_accounts,
                {
                    "commitment": "confirmed",
                    "encoding": "base64",
                    "dataSlice": {
                        "offset": 0,
                        "length": 0
                    }
                }
            ],
        });

        let response = self
            .client
            .post(self.rpc_url.clone())
            .json(&payload)
            .send()
            .await?;
        let body: RpcEnvelope<RpcAccountsResult> = response.error_for_status()?.json().await?;

        body.result.ok_or_else(|| {
            SourceError::MalformedResponse("missing getMultipleAccounts result".to_owned())
        })
    }
}

#[async_trait]
impl Source for SolanaRpcSource {
    fn name(&self) -> &'static str {
        "solana_rpc"
    }

    async fn fetch(&self) -> Result<Vec<EventEnvelope>, SourceError> {
        let observed_at = SystemTime::now();
        let slot = self.fetch_slot().await?;
        let accounts = self.fetch_accounts().await?;

        // Emit one event per meaningful observation so downstream consumers can
        // filter or aggregate without unpacking nested source responses.
        let mut events = Vec::with_capacity(1 + accounts.value.len());
        events.push(EventEnvelope::new(
            EventSource::SolanaRpc,
            EventPayload::RpcSlot { slot },
            observed_at,
        ));

        for (pubkey, account) in self
            .monitored_accounts
            .iter()
            .zip(accounts.value.into_iter())
        {
            let account = account.ok_or_else(|| {
                SourceError::MalformedResponse(format!("account not found for pubkey `{pubkey}`"))
            })?;

            events.push(EventEnvelope::new(
                EventSource::SolanaRpc,
                EventPayload::AccountSnapshot {
                    pubkey: pubkey.clone(),
                    slot: accounts.context.slot,
                    lamports: account.lamports,
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                    data_len: account.space.unwrap_or_default(),
                },
                observed_at,
            ));
        }

        Ok(events)
    }
}

#[derive(Debug, Deserialize)]
struct RpcEnvelope<T> {
    result: Option<T>,
}

#[derive(Debug, Deserialize)]
struct RpcAccountsResult {
    context: RpcContext,
    value: Vec<Option<RpcAccount>>,
}

#[derive(Debug, Deserialize)]
struct RpcContext {
    slot: u64,
}

#[derive(Debug, Deserialize)]
struct RpcAccount {
    lamports: u64,
    owner: String,
    executable: bool,
    #[serde(rename = "rentEpoch")]
    rent_epoch: u64,
    space: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::{RpcAccountsResult, RpcEnvelope};

    #[test]
    fn parses_slot_envelope() {
        let payload = r#"{"jsonrpc":"2.0","result":349002101,"id":1}"#;
        let parsed: RpcEnvelope<u64> = serde_json::from_str(payload).expect("valid rpc response");

        assert_eq!(parsed.result, Some(349002101));
    }

    #[test]
    fn parses_multiple_accounts_response() {
        let payload = r#"
        {
          "jsonrpc": "2.0",
          "result": {
            "context": { "slot": 349002101 },
            "value": [
              {
                "lamports": 10,
                "owner": "owner-1",
                "executable": false,
                "rentEpoch": 18446744073709551615,
                "space": 512
              }
            ]
          },
          "id": 2
        }
        "#;

        let parsed: RpcEnvelope<RpcAccountsResult> =
            serde_json::from_str(payload).expect("valid account response");

        let result = parsed.result.expect("result present");
        assert_eq!(result.context.slot, 349002101);
        assert_eq!(result.value.len(), 1);
        assert_eq!(
            result.value[0].as_ref().expect("account present").space,
            Some(512)
        );
    }
}
