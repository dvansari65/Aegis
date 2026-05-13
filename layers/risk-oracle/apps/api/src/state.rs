use reqwest::Client;

use crate::config::{ApiConfig, ChainMonitorConfig};
use crate::error::ApiError;

#[derive(Debug, Clone)]
pub struct AppState {
    pub http: Client,
    pub chain: Option<ChainMonitorConfig>,
}

impl AppState {
    pub fn new(config: &ApiConfig) -> Result<Self, ApiError> {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(12))
            .build()
            .map_err(|e| ApiError::Config(format!("http client: {e}")))?;

        Ok(Self {
            http,
            chain: config.chain.clone(),
        })
    }
}
