use lib::app::{build_clickhouse_service, build_grpc_server, build_log_row_service, build_mscp_channel, build_redis_service, build_worker_business};
use lib::config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = AppConfig::default();

    let clickhouse_service = build_clickhouse_service(config.clickhouse).await;
    let redis_service = build_redis_service(config.redis);

    let (tx, rx) = build_mscp_channel(&config.worker);

    let log_row_service = build_log_row_service(&clickhouse_service, &redis_service);
    let worker = build_worker_business(&config.worker, rx, log_row_service);
    worker.start().await;

    let server = build_grpc_server(config.grpc);
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
                .add_directive("hyper_util::client=warn".parse().unwrap())
        )
        .init();

    tracing::debug!("Tracing initialized");
}
