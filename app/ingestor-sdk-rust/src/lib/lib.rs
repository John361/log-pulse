mod client;
mod layer;
mod message_visitor;

// How to initialize
// #[tokio::main]
// async fn main() {
//     let log_pulse_ingestor_layer = LogPulseIngestorLayer::new("http://[::1]:8080".to_string(), "my-service".to_string()).await;
//
//     tracing_subscriber::registry()
//         .with(tracing_subscriber::fmt::layer())
//         .with(log_pulse_ingestor_layer)
//         .init();
//
//     tracing::info!("Hello LogPulseIngestor!");
// }
