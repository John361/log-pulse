use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming};
use tonic::codegen::tokio_stream::StreamExt;

use crate::grpc::log::log_ingestor_grpc_server::LogIngestorGrpc;
use crate::grpc::log::{LogEntryRequest, LogResponse};

#[derive(Clone)]
pub struct LogIngestorServiceGrpcImpl {
    tx: mpsc::Sender<LogEntryRequest>,
}

impl LogIngestorServiceGrpcImpl {
    pub fn new(tx: mpsc::Sender<LogEntryRequest>) -> Self {
        Self { tx }
    }
}

#[tonic::async_trait]
impl LogIngestorGrpc for LogIngestorServiceGrpcImpl {
    async fn send_log(&self, request: Request<LogEntryRequest>) -> Result<Response<LogResponse>, Status> {
        let log_entry = request.into_inner();

        if let Err(e) = self.tx.try_send(log_entry) {
            return Err(Status::resource_exhausted(format!("Buffer full: {}", e)));
        }

        Ok(Response::new(LogResponse {
            success: true,
            error_message: String::new(),
        }))
    }

    async fn stream_logs(&self, request: Request<Streaming<LogEntryRequest>>) -> Result<Response<LogResponse>, Status> {
        let mut stream = request.into_inner();

        while let Some(log_entry) = stream.next().await {
            let log_entry = log_entry?;
            let _ = self.tx.send(log_entry).await;
        }

        Ok(Response::new(LogResponse {
            success: true,
            error_message: String::new(),
        }))
    }
}
