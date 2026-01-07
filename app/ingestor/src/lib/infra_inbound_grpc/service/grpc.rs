use tokio::sync::mpsc::Sender;
use tonic::transport::Server;

use crate::config::GrpcConfig;
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

        tracing::info!("LogPulse Ingestor listening on {}", self.config.uri());

        Server::builder()
            .add_service(LogIngestorGrpcServer::new(service))
            .serve(self.config.uri())
            .await
            .inspect(|_| tracing::info!("GRPC server started"))
            .unwrap_or_else(|e| panic!("Cannot start GRPC server: {e:}"));
    }
}
