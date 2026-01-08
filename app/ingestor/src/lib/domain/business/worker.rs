use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::interval;

use crate::domain::service::LogRowService;
use crate::grpc::log::LogEntryRequest;

pub struct WorkerBusiness {
    rx: mpsc::Receiver<LogEntryRequest>,
    flush_interval_seconds: u64,
    batch_capacity: usize,
    service: LogRowService,
}

impl WorkerBusiness {
    pub fn new(rx: mpsc::Receiver<LogEntryRequest>, flush_interval_seconds: u64, batch_capacity: usize, service: LogRowService) -> Self {
        Self {
            rx,
            flush_interval_seconds,
            batch_capacity,
            service,
        }
    }

    pub async fn start(mut self) {
        tokio::spawn(async move {
            let mut flush_interval = interval(Duration::from_secs(self.flush_interval_seconds));
            let mut local_batch = Vec::with_capacity(self.batch_capacity);

            loop {
                tokio::select! {
                    Some(log) = self.rx.recv() => {
                        local_batch.push(log.into());

                        if local_batch.len() >= self.batch_capacity {
                            if let Err(e) = self.service.push_to_buffer(local_batch.drain(..).collect()).await {
                                tracing::error!("Failed to push batch to buffer: {:?}", e);
                            }
                        }
                    }

                    _ = flush_interval.tick() => {
                        if !local_batch.is_empty() {
                            if let Err(e) = self.service.push_to_buffer(local_batch.drain(..).collect()).await {
                                tracing::error!("Failed to push remaining batch to buffer: {:?}", e);
                            }
                        }

                        tracing::info!("Flushing batch due to interval...");

                        if let Err(e) = self.service.flush().await {
                            tracing::error!("Failed to flush logs: {e:?}. Retrying at next interval.");
                        }
                    }
                }
            }
        });
    }
}
