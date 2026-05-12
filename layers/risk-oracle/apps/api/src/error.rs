use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("server error: {0}")]
    Server(#[from] axum::Error),
}
