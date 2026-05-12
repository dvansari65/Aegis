use std::{env, net::SocketAddr};

use crate::error::ApiError;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub bind_address: SocketAddr,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self, ApiError> {
        let bind_address = env::var("RISK_ORACLE_API_BIND")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
            .parse::<SocketAddr>()
            .map_err(|error| {
                ApiError::Config(format!("invalid RISK_ORACLE_API_BIND value: {error}"))
            })?;

        Ok(Self { bind_address })
    }
}
