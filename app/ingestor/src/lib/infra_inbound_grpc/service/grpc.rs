use tokio::sync::mpsc::Sender;
use tonic::transport::Server;
use tonic_health::server::health_reporter;
use tonic_health::ServingStatus;

use crate::config::GrpcConfig;
use crate::grpc::log;
use crate::grpc::log::log_ingestor_grpc_server::LogIngestorGrpcServer;
use crate::grpc::log::LogEntryRequest;
use crate::infra_inbound_grpc::service::LogIngestorServiceGrpcImpl;

pub struct GrpcServer {
    config: GrpcConfig,
}

impl GrpcServer {
    pub fn new(config: GrpcConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, tx: Sender<LogEntryRequest>) {
        let service = LogIngestorServiceGrpcImpl::new(tx);

        let (health_reporter, health_service) = health_reporter();
        health_reporter.set_service_status("log-pulse-ingestor", ServingStatus::Serving).await;

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(log::FILE_DESCRIPTOR_SET)
            .register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
            .build_v1()
            .unwrap_or_else(|e| panic!("Cannot start GRPC server with reflection: {e:}"));

        tracing::info!("LogPulse Ingestor listening on {}", self.config.uri());

        Server::builder()
            .add_service(health_service)
            .add_service(reflection_service)
            .add_service(LogIngestorGrpcServer::new(service))
            .serve(self.config.uri())
            .await
            .inspect(|_| tracing::info!("GRPC server started"))
            .unwrap_or_else(|e| panic!("Cannot start GRPC server: {e:}"));
    }
}
