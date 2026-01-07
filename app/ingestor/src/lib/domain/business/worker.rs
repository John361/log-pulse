use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::interval;

use crate::domain::model::LogRow;
use crate::domain::service::LogRowService;
use crate::grpc::log::LogEntryRequest;

pub struct WorkerBusiness {
    rx: mpsc::Receiver<LogEntryRequest>,
    batch_capacity: usize,
    flush_interval_seconds: u64,
    service: LogRowService,
}

impl WorkerBusiness {
    pub fn new(rx: mpsc::Receiver<LogEntryRequest>, batch_capacity: usize, flush_interval_seconds: u64, service: LogRowService) -> Self {
        Self {
            rx,
            batch_capacity,
            flush_interval_seconds,
            service,
        }
    }

    pub async fn start(mut self) {
        tokio::spawn(async move {
            let mut batch = Vec::with_capacity(self.batch_capacity);
            let mut flush_interval = interval(Duration::from_secs(self.flush_interval_seconds));

            loop {
                tokio::select! {
                    Some(log) = self.rx.recv() => {
                        batch.push(log);
                        if batch.len() >= self.batch_capacity {
                            tracing::info!("Batch full, flushing...");
                            self.flush_batch(&mut batch).await;
                        }
                    }

                    _ = flush_interval.tick() => {
                        if !batch.is_empty() {
                            tracing::info!("Flushing batch due to interval...");
                            self.flush_batch(&mut batch).await;
                        }
                    }
                }
            }
        });
    }

    async fn flush_batch(&self, batch: &mut Vec<LogEntryRequest>) {
        if batch.is_empty() {
            return;
        }

        let mut values = Vec::new();

        for entry in batch.drain(..) {
            let row = LogRow::from(entry);
            values.push(row);
        }

        self.service.insert(values).await.unwrap(); // TODO: fix unwrap
    }
}
