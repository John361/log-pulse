use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::{Request, Response, Status, Streaming};
use lib::grpc::log::log_ingestor_grpc_server::LogIngestorGrpc;
use lib::grpc::log::{LogEntryRequest, LogResponse};

#[derive(Default)]
pub struct LogPulseIngestorMock {
    pub count: Arc<Mutex<usize>>,
}

#[tonic::async_trait]
impl LogIngestorGrpc for LogPulseIngestorMock {
    async fn send_log(&self, _req: Request<LogEntryRequest>) -> Result<Response<LogResponse>, Status> {
        Ok(Response::new(LogResponse { success: true, error_message: "".to_string() }))
    }

    async fn stream_logs(&self, request: Request<Streaming<LogEntryRequest>>) -> Result<Response<LogResponse>, Status> {
        let mut stream = request.into_inner();
        let counter = Arc::clone(&self.count);

        tokio::spawn(async move {
            while let Ok(Some(log)) = stream.message().await {
                let mut c = counter.lock().await;
                *c += 1;
                println!("[LogPulseIngestorMock] Log received #{}: {}", *c, log.message);
            }
        });

        Ok(Response::new(LogResponse { success: true, error_message: "".to_string() }))
    }
}
