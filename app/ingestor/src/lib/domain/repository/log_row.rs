use anyhow::Result;
use async_trait::async_trait;

use crate::domain::model::LogRow;

#[async_trait]
pub trait LogRowRepository: Send + Sync + 'static {
    async fn insert(&self, values: Vec<LogRow>) -> Result<()>;
}

#[async_trait]
pub trait LogRowBufferRepository: Send + Sync + 'static {
    async fn push(&self, value: LogRow) -> Result<()>;
    async fn flush(&self) -> Result<Vec<LogRow>>;
}
