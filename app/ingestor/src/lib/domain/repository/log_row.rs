use anyhow::Result;
use async_trait::async_trait;

use crate::domain::model::LogRow;

#[async_trait]
pub trait LogRowRepository: Send + Sync + 'static {
    async fn insert(&self, values: Vec<LogRow>) -> Result<()>;
}
