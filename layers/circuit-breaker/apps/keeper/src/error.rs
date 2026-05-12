use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeeperError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("shutdown listener failed: {0}")]
    Shutdown(#[from] std::io::Error),
}
