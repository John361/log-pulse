use tokio::sync::mpsc::Sender;
use tonic::transport::Server;

use crate::grpc::log::log_ingestor_grpc_server::LogIngestorGrpcServer;
use crate::grpc::log::LogEntryRequest;
use crate::infra_inbound_grpc::service::LogIngestorServiceGrpcImpl;

pub struct GrpcServer {

}

impl GrpcServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(&self, tx: Sender<LogEntryRequest>) {
        let addr = "[::1]:50051".parse().unwrap();
        let service = LogIngestorServiceGrpcImpl::new(tx);

        tracing::info!("LogPulse Ingestor listening on {}", addr);

        Server::builder()
            .add_service(LogIngestorGrpcServer::new(service))
            .serve(addr)
            .await
            .inspect(|_| tracing::info!("GRPC server started"))
            .unwrap_or_else(|e| panic!("Cannot start GRPC server: {e:}"));
    }
}
