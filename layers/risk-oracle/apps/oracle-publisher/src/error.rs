use thiserror::Error;

#[derive(Debug, Error)]
pub enum PublisherError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("shutdown listener failed: {0}")]
    Shutdown(#[from] std::io::Error),
}
