mod mock;

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use ingestor_sdk_lib::layer::LogPulseIngestorLayer;
use lib::grpc::log::log_ingestor_grpc_server::LogIngestorGrpcServer;

use crate::mock::LogPulseIngestorMock;

#[tokio::test]
async fn test_with_mock_server() -> Result<()> {
    let addr = "[::1]:50051".parse()?;
    let mock_service = LogPulseIngestorMock::default();
    let counter = Arc::clone(&mock_service.count);

    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(LogIngestorGrpcServer::new(mock_service))
            .serve(addr)
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let layer = LogPulseIngestorLayer::new(
        "http://[::1]:50051".to_string(),
        "ingestor-sdk-rust-integration-test-service".to_string()
    ).await?;

    let _ = tracing_subscriber::registry().with(layer).try_init();

    for i in 0..10 {
        tracing::info!("Log test {}", i);
    }

    tokio::time::sleep(Duration::from_millis(200)).await;
    let final_count = *counter.lock().await;

    println!("Total received logs from mock: {}", final_count);
    assert!(final_count >= 10);

    Ok(())
}
