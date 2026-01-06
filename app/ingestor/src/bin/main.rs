use anyhow::Result;
use tokio::sync::mpsc;

use lib::domain::business::WorkerBusiness;
use lib::grpc::log::LogEntryRequest;
use lib::infra_inbound_grpc::service::GrpcServer;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let buffer_size = 10000;
    let (tx, rx) = mpsc::channel::<LogEntryRequest>(buffer_size);

    let ch_url = "http://localhost:8123"; // TODO: move in config
    let ch_user = "log_pulse";
    let ch_pass = "changeme";
    let ch_db = "log_pulse";
    let client = clickhouse::Client::default()
        .with_url(ch_url)
        .with_user(ch_user)
        .with_password(ch_pass)
        .with_database(ch_db);

    let worker = WorkerBusiness::new(rx, 100, 5, client);
    worker.start().await;

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
