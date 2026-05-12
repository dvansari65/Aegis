use clap::Parser;
use std::{num::NonZeroU64, time::Duration};
use url::Url;

use crate::error::IngestionError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    // Solana RPC endpoint used for account and slot polling.
    pub rpc_url: Url,
    // Accounts are configured explicitly so downstream scoring can focus on
    // a small set of known liquidity and risk-critical addresses first.
    pub monitored_accounts: Vec<String>,
    pub pyth_hermes_url: Option<Url>,
    pub pyth_feed_ids: Vec<String>,
    pub poll_interval: Duration,
}

#[derive(Debug, Parser)]
#[command(
    name = "shock-absorber-ingestion",
    about = "Data ingestion service for the Solana stablecoin shock absorber"
)]
pub struct Cli {
    #[arg(long, env = "SFA_RPC_URL")]
    rpc_url: String,
    #[arg(long, env = "SFA_MONITORED_ACCOUNTS", value_delimiter = ',')]
    monitored_accounts: Vec<String>,
    #[arg(long, env = "SFA_PYTH_HERMES_URL")]
    pyth_hermes_url: Option<String>,
    #[arg(long, env = "SFA_PYTH_FEED_IDS", value_delimiter = ',')]
    pyth_feed_ids: Vec<String>,
    #[arg(
        long,
        env = "SFA_POLL_INTERVAL_MS",
        default_value_t = NonZeroU64::new(2_000).expect("poll interval default must be non-zero")
    )]
    poll_interval_ms: NonZeroU64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, IngestionError> {
        Self::try_from(Cli::parse())
    }
}

impl TryFrom<Cli> for AppConfig {
    type Error = IngestionError;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        let rpc_url = Url::parse(&cli.rpc_url)
            .map_err(|err| IngestionError::InvalidConfig(format!("SFA_RPC_URL: {err}")))?;

        // The service is only meaningful if at least one upstream signal source
        // is configured.
        if cli.monitored_accounts.is_empty() && cli.pyth_feed_ids.is_empty() {
            return Err(IngestionError::InvalidConfig(
                "configure at least one monitored account or one Pyth feed".to_owned(),
            ));
        }

        let pyth_hermes_url = cli
            .pyth_hermes_url
            .as_deref()
            .map(Url::parse)
            .transpose()
            .map_err(|err| IngestionError::InvalidConfig(format!("SFA_PYTH_HERMES_URL: {err}")))?;

        Ok(Self {
            rpc_url,
            monitored_accounts: normalize_list(cli.monitored_accounts),
            pyth_hermes_url,
            pyth_feed_ids: normalize_list(cli.pyth_feed_ids),
            poll_interval: Duration::from_millis(cli.poll_interval_ms.get()),
        })
    }
}

fn normalize_list(values: Vec<String>) -> Vec<String> {
    // Normalize comma-separated env values into stable identifiers before the
    // rest of the system consumes them.
    values
        .into_iter()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{AppConfig, Cli};

    #[test]
    fn rejects_empty_source_configuration() {
        let cli = Cli {
            rpc_url: "https://api.mainnet-beta.solana.com".to_owned(),
            monitored_accounts: vec![],
            pyth_hermes_url: None,
            pyth_feed_ids: vec![],
            poll_interval_ms: 1000.try_into().expect("non-zero"),
        };

        let result = AppConfig::try_from(cli);

        assert!(result.is_err());
    }

    #[test]
    fn trims_input_lists() {
        let cli = Cli {
            rpc_url: "https://api.mainnet-beta.solana.com".to_owned(),
            monitored_accounts: vec![" account-a ".to_owned(), "".to_owned()],
            pyth_hermes_url: Some("https://hermes.pyth.network".to_owned()),
            pyth_feed_ids: vec![" feed-1 ".to_owned()],
            poll_interval_ms: 1000.try_into().expect("non-zero"),
        };

        let config = AppConfig::try_from(cli).expect("valid config");

        assert_eq!(config.monitored_accounts, vec!["account-a"]);
        assert_eq!(config.pyth_feed_ids, vec!["feed-1"]);
    }
}
