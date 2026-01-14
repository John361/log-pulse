use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use prost_types::Timestamp;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tracing::Subscriber;
use tracing_subscriber::Layer;

use lib::grpc::log::LogEntryRequest;

use crate::client::LogPulseClient;
use crate::message_visitor::MessageVisitor;

pub struct LogPulseIngestorLayer {
    tx: Sender<LogEntryRequest>,
    service_name: String,
}

impl LogPulseIngestorLayer {
    pub async fn new(endpoint: String, service_name: String) -> Result<Self> {
        let client = LogPulseClient::connect(endpoint).await?;
        let (tx, rx) = mpsc::channel::<LogEntryRequest>(1024);
        client.start_streaming(rx).await;

        Ok(Self {
            tx,
            service_name,
        })
    }

    fn timestamp_now(&self) -> Option<Timestamp> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Some(Timestamp {
            seconds: now.as_secs() as i64,
            nanos: now.subsec_nanos() as i32,
        })
    }
}

impl<S> Layer<S> for LogPulseIngestorLayer where S: Subscriber {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut visitor = MessageVisitor { message: String::new() };
        event.record(&mut visitor);

        if visitor.message.is_empty() {
            return;
        }

        let level = match *event.metadata().level() {
            tracing::Level::DEBUG => 0,
            tracing::Level::TRACE => 0,
            tracing::Level::INFO => 1,
            tracing::Level::WARN => 2,
            tracing::Level::ERROR => 3,
        };

        let log_entry = LogEntryRequest {
            timestamp: self.timestamp_now(),
            level,
            service_name: self.service_name.clone(),
            message: visitor.message,
            metadata: None,
        };

        if let Err(e) = self.tx.try_send(log_entry) {
            eprintln!("[LogPulseIngestor-SDK] Internal buffer full, dropping log: {}", e);
        }
    }
}
