use async_trait::async_trait;
use tokio::{
    sync::mpsc,
    task::JoinHandle,
    time::{MissedTickBehavior, interval},
};
use tracing::{error, info, warn};

use crate::{
    config::AppConfig,
    error::IngestionError,
    models::EventEnvelope,
    sink::JsonStdoutSink,
    sources::{Source, create_sources},
};

pub struct DataIngestionService {
    config: AppConfig,
}

impl DataIngestionService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub async fn run(self) -> Result<(), IngestionError> {
        // Source construction is config-driven so we can add new upstream data
        // feeds without changing the service bootstrap flow.
        let sources = create_sources(&self.config).await?;
        let (sender, mut receiver) = mpsc::channel::<EventEnvelope>(1024);

        let mut tasks = Vec::with_capacity(sources.len());
        for source in sources {
            tasks.push(spawn_source_task(
                source,
                self.config.poll_interval,
                sender.clone(),
            ));
        }
        drop(sender);

        let sink_task = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                JsonStdoutSink::emit(&event);
            }
        });

        tokio::signal::ctrl_c()
            .await
            .map_err(|err| IngestionError::SourceInit {
                component: "runtime".to_owned(),
                message: format!("failed to listen for shutdown signal: {err}"),
            })?;

        info!("shutdown signal received");

        for task in tasks {
            task.abort();
        }
        sink_task.abort();

        Ok(())
    }
}

fn spawn_source_task(
    source: Box<dyn Source>,
    poll_interval: std::time::Duration,
    sender: mpsc::Sender<EventEnvelope>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        info!(source = source.name(), "starting source loop");

        let mut ticker = interval(poll_interval);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            ticker.tick().await;

            match source.fetch().await {
                Ok(events) => {
                    // Each source can emit multiple normalized events from one
                    // poll cycle, such as one slot plus several account reads.
                    for event in events {
                        if sender.send(event).await.is_err() {
                            warn!(source = source.name(), "event channel closed");
                            return;
                        }
                    }
                }
                Err(err) => {
                    error!(source = source.name(), error = %err, "source fetch failed");
                }
            }
        }
    })
}

#[async_trait]
pub trait RunnableService {
    async fn run(self) -> Result<(), IngestionError>;
}

#[async_trait]
impl RunnableService for DataIngestionService {
    async fn run(self) -> Result<(), IngestionError> {
        DataIngestionService::run(self).await
    }
}
