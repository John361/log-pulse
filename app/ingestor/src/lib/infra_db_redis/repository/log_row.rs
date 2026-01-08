use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::domain::model::LogRow;
use crate::domain::repository::LogRowBufferRepository;
use crate::infra_db_redis::service::RedisService;

pub struct LogRowBufferRedisRepository {
    service: Arc<RedisService>,
}

impl LogRowBufferRedisRepository {
    pub fn new(service: Arc<RedisService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl LogRowBufferRepository for LogRowBufferRedisRepository {
    async fn push(&self, values: Vec<LogRow>) -> Result<()> {
        let mut connection = self.service.connection().clone();

        let mut pipe = redis::pipe();
        for value in values {
            let log_json = serde_json::to_string(&value)?;
            pipe.cmd("RPUSH").arg(&self.service.database).arg(log_json);
        }

        let _: () = pipe.query_async(&mut connection).await?;

        Ok(())
    }

    async fn flush(&self) -> Result<Vec<LogRow>> {
        let mut connection = self.service.connection().clone();

        let logs: Vec<String> = redis::cmd("LPOP")
            .arg(&self.service.database)
            .arg(self.service.batch_capacity)
            .query_async(&mut connection)
            .await?;

        let logs = logs.iter()
            .filter_map(|json| serde_json::from_str::<LogRow>(json).ok())
            .collect();

        Ok(logs)
    }
}
