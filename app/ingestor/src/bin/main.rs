use anyhow::Result;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

use lib::grpc::log::LogEntryRequest;
use lib::infra_inbound_grpc::service::GrpcServer;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let batch_capacity = 100;
    let buffer_size = 10000;
    let flush_interval_seconds = 5;

    let (tx, mut rx) = mpsc::channel::<LogEntryRequest>(buffer_size);

    tokio::spawn(async move {
        let mut batch = Vec::with_capacity(batch_capacity);
        let mut flush_interval = interval(Duration::from_secs(flush_interval_seconds));

        loop {
            tokio::select! {
                Some(log) = rx.recv() => {
                    batch.push(log);
                    if batch.len() >= batch_capacity {
                        tracing::info!("Batch full, flushing...");
                        flush_batch(&mut batch).await;
                    }
                }

                _ = flush_interval.tick() => {
                    if !batch.is_empty() {
                        tracing::info!("Flushing batch due to interval...");
                        flush_batch(&mut batch).await;
                    }
                }
            }
        }
    });

    let server = GrpcServer::new();
    server.start(tx).await;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("h2::codec=warn".parse().unwrap())
                .add_directive("h2::proto=warn".parse().unwrap())
        )
        .init();

    tracing::debug!("Tracing initialized");
}

async fn flush_batch(batch: &mut Vec<LogEntryRequest>) {
    let count = batch.len();

    tracing::info!("--- Flushing batch of {} logs to storage ---", count);

    batch.clear();
}
