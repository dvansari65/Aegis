//! Circuit breaker keeper: reads risk oracle account data, runs policy, plans `UpdatePolicy` bytes.
//!
//! On-chain transaction submission is a separate step (signer + accounts). See `pipeline` tests
//! for deterministic end-to-end wiring of **account bytes → policy → instruction encoding**.

pub mod cb_account;
pub mod config;
pub mod error;
pub mod ix;
pub mod keeper;
pub mod plan;
pub mod pipeline;
pub mod rpc;

pub use config::KeeperConfig;
pub use error::KeeperError;
pub use keeper::KeeperWorker;
pub use plan::PolicyWireSummary;
pub use pipeline::{run_policy_tick_from_accounts, PolicyTickOutput};

use tracing_subscriber::{EnvFilter, fmt};

/// Initialize tracing once (safe if called multiple times).
pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .json()
        .try_init();
}
