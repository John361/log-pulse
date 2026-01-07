use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};

use crate::config::WorkerConfig;
use crate::domain::business::WorkerBusiness;
use crate::domain::repository::LogRowRepository;
use crate::domain::service::LogRowService;
use crate::grpc::log::LogEntryRequest;
use crate::infra_db_clickhouse::repository::LogRowClickhouseRepository;
use crate::infra_db_clickhouse::service::ClickhouseService;

pub fn build_log_row_service(service: &Arc<Mutex<ClickhouseService>>) -> LogRowService {
    let repository = LogRowClickhouseRepository::new(service.clone());
    let repository: Box<dyn LogRowRepository> = Box::new(repository);
    let repository = Arc::new(Mutex::new(repository));

    LogRowService::new(repository)
}

pub fn build_mscp_channel(config: &WorkerConfig) -> (Sender<LogEntryRequest>, Receiver<LogEntryRequest>) {
    mpsc::channel::<LogEntryRequest>(config.buffer_size)
}

pub fn build_worker_business(config: &WorkerConfig, rx: Receiver<LogEntryRequest>, service: LogRowService) -> WorkerBusiness {
    WorkerBusiness::new(rx, config.batch_capacity, config.flush_interval_seconds, service)
}
