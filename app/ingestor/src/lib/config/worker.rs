use serde::Deserialize;

#[derive(Deserialize)]
pub struct WorkerConfig {
    pub buffer_size: usize,
    pub flush_interval_seconds: u64,
}
