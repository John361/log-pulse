use anyhow::Result;
use tokio::sync::mpsc;

use lib::config::AppConfig;
use lib::domain::business::WorkerBusiness;
use lib::grpc::log::LogEntryRequest;
use lib::infra_inbound_grpc::service::GrpcServer;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let config = AppConfig::default();
    let clickhouse_client = clickhouse::Client::default()
        .with_url(config.clickhouse.address)
        .with_user(config.clickhouse.user)
        .with_password(config.clickhouse.password)
        .with_database(config.clickhouse.database);

    let (tx, rx) = mpsc::channel::<LogEntryRequest>(config.worker.buffer_size);
    let worker = WorkerBusiness::new(rx, config.worker.batch_capacity, config.worker.flush_interval_seconds, clickhouse_client);
    worker.start().await;

    let server = GrpcServer::new(config.grpc);
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
