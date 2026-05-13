use std::{env, net::SocketAddr};

use tracing::warn;

use crate::error::ApiError;

#[derive(Debug, Clone)]
pub struct ChainMonitorConfig {
    pub rpc_url: String,
    pub risk_pubkey: String,
    pub cb_pubkey: Option<String>,
    pub oracle_confidence: u8,
}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub bind_address: SocketAddr,
    pub chain: Option<ChainMonitorConfig>,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self, ApiError> {
        let bind_address = env::var("RISK_ORACLE_API_BIND")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
            .parse::<SocketAddr>()
            .map_err(|error| {
                ApiError::Config(format!("invalid RISK_ORACLE_API_BIND value: {error}"))
            })?;

        let rpc = env::var("RISK_ORACLE_MONITOR_SOLANA_RPC_URL")
            .ok()
            .filter(|s| !s.trim().is_empty());
        let risk_pk = env::var("RISK_ORACLE_MONITOR_STATE_PUBKEY")
            .ok()
            .filter(|s| !s.trim().is_empty());
        let cb_pk = env::var("RISK_ORACLE_MONITOR_CB_STATE_PUBKEY")
            .ok()
            .filter(|s| !s.trim().is_empty());

        let chain = match (rpc, risk_pk) {
            (Some(rpc_url), Some(risk_pubkey)) => {
                let oracle_confidence = env::var("RISK_ORACLE_MONITOR_ORACLE_CONFIDENCE")
                    .ok()
                    .and_then(|s| s.parse::<u8>().ok())
                    .unwrap_or(100);

                Some(ChainMonitorConfig {
                    rpc_url,
                    risk_pubkey,
                    cb_pubkey: cb_pk,
                    oracle_confidence,
                })
            }
            (None, None) => None,
            (Some(_), None) => {
                warn!("RISK_ORACLE_MONITOR_SOLANA_RPC_URL is set without RISK_ORACLE_MONITOR_STATE_PUBKEY; chain monitor disabled");
                None
            }
            (None, Some(_)) => {
                warn!("RISK_ORACLE_MONITOR_STATE_PUBKEY is set without RISK_ORACLE_MONITOR_SOLANA_RPC_URL; chain monitor disabled");
                None
            }
        };

        Ok(Self {
            bind_address,
            chain,
        })
    }
}
