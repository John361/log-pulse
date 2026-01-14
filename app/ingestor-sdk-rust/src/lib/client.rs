use std::time::Duration;

use anyhow::Result;

use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::ReceiverStream;

use lib::grpc::log::log_ingestor_grpc_client::LogIngestorGrpcClient;
use lib::grpc::log::LogEntryRequest;

#[derive(Clone)]
pub struct LogPulseClient {
    client: LogIngestorGrpcClient<tonic::transport::Channel>,
}

impl LogPulseClient {
    pub async fn connect(endpoint: String) -> Result<Self> {
        let channel = tonic::transport::Endpoint::from_shared(endpoint)?
            .connect_lazy();

        Ok(Self {
            client: LogIngestorGrpcClient::new(channel),
        })
    }

    pub async fn start_streaming(self, mut rx: Receiver<LogEntryRequest>) {
        tokio::spawn(async move {
            loop {
                let (tx_inner, rx_inner) = mpsc::channel(128);
                let mut client = self.client.clone();
                let stream = ReceiverStream::new(rx_inner);
                let rpc_handle = tokio::spawn(async move {
                    client.stream_logs(stream).await
                });

                while let Some(log) = rx.recv().await {
                    if tx_inner.send(log).await.is_err() {
                        break;
                    }
                }

                if let Err(e) = rpc_handle.await {
                    eprintln!("[LogPulseIngestor-SDK] RPC Task failed: {e:?}");
                }

                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }
}
