use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::model::LogRow;
use crate::domain::repository::LogRowRepository;

#[derive(Clone)]
pub struct LogRowService {
    repository: Arc<Mutex<Box<dyn LogRowRepository>>>,
}

impl LogRowService {
    pub fn new(repository: Arc<Mutex<Box<dyn LogRowRepository>>>) -> Self {
        Self { repository }
    }

    pub async fn insert(&self, values: Vec<LogRow>) -> anyhow::Result<()> {
        let repository = self.repository.lock().await;
        repository.insert(values).await
    }
}
