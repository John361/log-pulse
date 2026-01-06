use std::time::Duration;

use clickhouse::Client;
use tokio::sync::mpsc;
use tokio::time::interval;

use crate::domain::model::LogRow;
use crate::grpc::log::LogEntryRequest;

pub struct WorkerBusiness {
    rx: mpsc::Receiver<LogEntryRequest>,
    batch_capacity: usize,
    flush_interval_seconds: u64,
    clickhouse_client: Client,
}

impl WorkerBusiness {
    pub fn new(rx: mpsc::Receiver<LogEntryRequest>, batch_capacity: usize, flush_interval_seconds: u64, clickhouse_client: Client) -> Self {
        Self {
            rx,
            batch_capacity,
            flush_interval_seconds,
            clickhouse_client,
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

    async fn flush_batch(&self, batch: &mut Vec<LogEntryRequest>) { // TODO: move in repository
        if batch.is_empty() {
            return;
        }

        let count = batch.len();
        let rows: Vec<LogRow> = batch.drain(..).map(LogRow::from).collect();
        let mut insert: clickhouse::insert::Insert<LogRow> = self.clickhouse_client.insert("logs").await.unwrap();

        for row in rows {
            if let Err(e) = insert.write(&row).await {
                tracing::error!("Failed to write row to ClickHouse buffer: {:?}", e);
            }
        }

        match insert.end().await {
            Ok(_) => tracing::info!("Successfully flushed {} logs to ClickHouse", count),
            Err(e) => tracing::error!("Failed to commit batch to ClickHouse: {:?}", e),
        }
    }
}
