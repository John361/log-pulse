use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::domain::model::LogRow;
use crate::domain::repository::{LogRowBufferRepository, LogRowRepository};

#[derive(Clone)]
pub struct LogRowService {
    repository: Arc<Mutex<Box<dyn LogRowRepository>>>,
    buffer_repository: Arc<Mutex<Box<dyn LogRowBufferRepository>>>,
}

impl LogRowService {
    pub fn new(repository: Arc<Mutex<Box<dyn LogRowRepository>>>, buffer_repository: Arc<Mutex<Box<dyn LogRowBufferRepository>>>) -> Self {
        Self { repository, buffer_repository }
    }

    pub async fn insert(&self, values: Vec<LogRow>) -> Result<()> {
        let repository = self.repository.lock().await;
        repository.insert(values).await
    }

    pub async fn push_to_buffer(&self, value: LogRow) -> Result<()> {
        let repository = self.buffer_repository.lock().await;
        repository.push(value).await
    }

    pub async fn flush(&self) -> Result<()> {
        Ok(())
    }
}
