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

pub struct DexPoolSource {
    client: Client,
    rpc_url: Url,
    pool_token_accounts: Vec<String>,
}

impl DexPoolSource {
    pub fn new(rpc_url: Url, pool_token_accounts: Vec<String>) -> Self {
        Self {
            client: Client::new(),
            rpc_url,
            pool_token_accounts,
        }
    }

    async fn fetch_token_balance(&self, account: &str) -> Result<(u64, u64), SourceError> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1u64,
            "method": "getTokenAccountBalance",
            "params": [
                account,
                { "commitment": "confirmed" }
            ],
        });

        let response = self
            .client
            .post(self.rpc_url.clone())
            .json(&payload)
            .send()
            .await?;
        let body: RpcEnvelope<TokenBalanceResult> = response.error_for_status()?.json().await?;

        let result = body.result
            .ok_or_else(|| SourceError::MalformedResponse("missing token balance".to_owned()))?;
        
        let amount = result.value.amount.parse::<u64>().unwrap_or(0);
        let slot = result.context.slot;

        Ok((amount, slot))
    }
}

#[async_trait]
impl Source for DexPoolSource {
    fn name(&self) -> &'static str {
        "dex_pools"
    }

    async fn fetch(&self) -> Result<Vec<EventEnvelope>, SourceError> {
        let observed_at = SystemTime::now();
        let mut events = Vec::with_capacity(self.pool_token_accounts.len());

        for account in &self.pool_token_accounts {
            if let Ok((balance, slot)) = self.fetch_token_balance(account).await {
                // Re-using AccountSnapshot where lamports = token balance for simplicity
                events.push(EventEnvelope::new(
                    EventSource::SolanaRpc,
                    EventPayload::AccountSnapshot {
                        pubkey: account.clone(),
                        slot,
                        lamports: balance,
                        owner: String::new(),
                        executable: false,
                        rent_epoch: 0,
                        data_len: 0,
                    },
                    observed_at,
                ));
            }
        }

        Ok(events)
    }
}

#[derive(Debug, Deserialize)]
struct RpcEnvelope<T> {
    result: Option<T>,
}

#[derive(Debug, Deserialize)]
struct TokenBalanceResult {
    context: RpcContext,
    value: TokenBalanceValue,
}

#[derive(Debug, Deserialize)]
struct RpcContext {
    slot: u64,
}

#[derive(Debug, Deserialize)]
struct TokenBalanceValue {
    amount: String,
}
