use anyhow::Result;
use tokio::sync::mpsc;

use lib::grpc::log::LogEntryRequest;
use lib::infra_inbound_grpc::service::GrpcServer;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let (tx, mut rx) = mpsc::channel::<LogEntryRequest>(10000);

    tokio::spawn(async move {
        while let Some(log) = rx.recv().await {
            tracing::info!("Storage worker received log from: {}", log.service_name);
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
