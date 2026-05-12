use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestionError {
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("failed to initialize component `{component}`: {message}")]
    SourceInit { component: String, message: String },
}

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json parsing failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("remote source returned malformed data: {0}")]
    MalformedResponse(String),
}
