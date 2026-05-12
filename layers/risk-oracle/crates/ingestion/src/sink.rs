use crate::models::EventEnvelope;

pub struct JsonStdoutSink;

impl JsonStdoutSink {
    pub fn emit(event: &EventEnvelope) {
        // Stdout is the simplest transport for early development and keeps the
        // ingestion layer decoupled from whichever database or queue we add next.
        if let Ok(line) = serde_json::to_string(event) {
            println!("{line}");
        }
    }
}
