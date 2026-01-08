use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};

use crate::config::WorkerConfig;
use crate::domain::business::WorkerBusiness;
use crate::domain::repository::{LogRowBufferRepository, LogRowRepository};
use crate::domain::service::LogRowService;
use crate::grpc::log::LogEntryRequest;
use crate::infra_db_clickhouse::repository::LogRowClickhouseRepository;
use crate::infra_db_clickhouse::service::ClickhouseService;
use crate::infra_db_redis::repository::LogRowBufferRedisRepository;
use crate::infra_db_redis::service::RedisService;

pub fn build_log_row_service(clickhouse_service: &Arc<Mutex<ClickhouseService>>, redis_service: &Arc<RedisService>) -> LogRowService {
    let repository = LogRowClickhouseRepository::new(clickhouse_service.clone());
    let repository: Box<dyn LogRowRepository> = Box::new(repository);
    let repository = Arc::new(Mutex::new(repository));

    let buffer_repository = LogRowBufferRedisRepository::new(redis_service.clone());
    let buffer_repository: Box<dyn LogRowBufferRepository> = Box::new(buffer_repository);
    let buffer_repository = Arc::new(Mutex::new(buffer_repository));

    LogRowService::new(repository, buffer_repository)
}

pub fn build_mscp_channel(config: &WorkerConfig) -> (Sender<LogEntryRequest>, Receiver<LogEntryRequest>) {
    mpsc::channel::<LogEntryRequest>(config.buffer_size)
}

pub fn build_worker_business(config: &WorkerConfig, rx: Receiver<LogEntryRequest>, service: LogRowService) -> WorkerBusiness {
    WorkerBusiness::new(rx, config.flush_interval_seconds, config.batch_capacity, service)
}
