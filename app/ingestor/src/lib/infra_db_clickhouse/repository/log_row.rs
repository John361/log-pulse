use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use clickhouse::insert::Insert;
use tokio::sync::Mutex;

use crate::domain::model::LogRow;
use crate::domain::repository::LogRowRepository;
use crate::infra_db_clickhouse::service::ClickhouseService;

pub struct LogRowClickhouseRepository {
    service: Arc<Mutex<ClickhouseService>>,
}

impl LogRowClickhouseRepository {
    pub fn new(service: Arc<Mutex<ClickhouseService>>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl LogRowRepository for LogRowClickhouseRepository {
    async fn insert(&self, values: Vec<LogRow>) -> Result<()> {
        let service = self.service.lock().await;

        let mut insert: Insert<LogRow> = match service.client().insert("logs").await {
            Ok(ins) => ins,
            Err(e) => {
                tracing::error!("Cannot initialize Clickhouse insert: {:?}", e);
                return Err(anyhow::anyhow!("Clickhouse insert error"));
            }
        };

        for value in &values {
            if let Err(e) = insert.write(value).await {
                tracing::error!("Write error in Clickhouse buffer: {:?}", e);
            }
        }

        match insert.end().await {
            Ok(_) => {
                tracing::info!("Successfully flushed {} logs to ClickHouse", values.len());
                Ok(())
            },
            Err(e) => {
                tracing::error!("Failed to commit batch to ClickHouse: {:?}", e);
                Err(anyhow::anyhow!("ClickHouse failed to commit batch to ClickHouse"))
            },
        }
    }
}
