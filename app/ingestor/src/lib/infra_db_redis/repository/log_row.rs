use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::domain::model::LogRow;
use crate::domain::repository::LogRowBufferRepository;
use crate::infra_db_redis::service::RedisService;

pub struct LogRowBufferRedisRepository {
    service: Arc<Mutex<RedisService>>,
}

impl LogRowBufferRedisRepository {
    pub fn new(service: Arc<Mutex<RedisService>>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl LogRowBufferRepository for LogRowBufferRedisRepository {
    async fn push(&self, value: LogRow) -> Result<()> {
        let service = self.service.lock().await;
        let mut connection = service.connection().await?;
        let log_json = serde_json::to_string(&value)?;

        let _: () = redis::cmd("RPUSH")
            .arg("ingestor:logs")
            .arg(log_json)
            .query_async(&mut connection)
            .await?;

        Ok(())
    }
}
