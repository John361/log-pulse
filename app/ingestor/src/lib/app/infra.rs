use std::sync::Arc;

use tokio::sync::Mutex;

use crate::config::{ClickhouseConfig, GrpcConfig};
use crate::infra_db_clickhouse::service::ClickhouseService;
use crate::infra_inbound_grpc::service::GrpcServer;

pub fn build_clickhouse_service(config: ClickhouseConfig) -> Arc<Mutex<ClickhouseService>> {
    let service = ClickhouseService::new(config);
    Arc::new(Mutex::new(service))
}

pub fn build_grpc_server(config: GrpcConfig) -> GrpcServer {
    GrpcServer::new(config)
}
