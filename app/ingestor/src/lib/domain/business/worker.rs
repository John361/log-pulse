use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::interval;

use crate::domain::service::LogRowService;
use crate::grpc::log::LogEntryRequest;

pub struct WorkerBusiness {
    rx: mpsc::Receiver<LogEntryRequest>,
    flush_interval_seconds: u64,
    service: LogRowService,
}

impl WorkerBusiness {
    pub fn new(rx: mpsc::Receiver<LogEntryRequest>, flush_interval_seconds: u64, service: LogRowService) -> Self {
        Self {
            rx,
            flush_interval_seconds,
            service,
        }
    }

    pub async fn start(mut self) {
        tokio::spawn(async move {
            let mut flush_interval = interval(Duration::from_secs(self.flush_interval_seconds));

            loop {
                tokio::select! {
                    Some(log) = self.rx.recv() => {
                        if let Err(e) = self.service.push_to_buffer(log.into()).await {
                            tracing::error!("Failed to push to buffer: {:?}", e);
                        }
                    }

                    _ = flush_interval.tick() => {
                        tracing::info!("Flushing batch due to interval...");
                        self.service.flush().await.unwrap(); // TODO: fix unwrap
                    }
                }
            }
        });
    }
}
