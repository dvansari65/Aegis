pub mod config;
pub mod error;
pub mod models;
pub mod pipeline;
pub mod sink;
pub mod sources;

pub use config::AppConfig;
pub use error::{IngestionError, SourceError};
pub use models::{EventEnvelope, EventPayload, EventSource};
pub use pipeline::DataIngestionService;
