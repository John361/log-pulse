use std::sync::Arc;

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

    pub async fn insert(&self, values: Vec<LogRow>) -> anyhow::Result<()> {
        let repository = self.repository.lock().await;
        repository.insert(values).await
    }
}
