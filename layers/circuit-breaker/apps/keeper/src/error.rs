use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeeperError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("shutdown listener failed: {0}")]
    Shutdown(#[from] std::io::Error),
    #[error("rpc error: {0}")]
    Rpc(String),
    #[error("risk oracle account: {0}")]
    RiskOracle(String),
    #[error("circuit breaker account: {0}")]
    CircuitBreaker(String),
}
