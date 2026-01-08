use std::sync::Arc;

use tokio::sync::Mutex;

use crate::config::{ClickhouseConfig, GrpcConfig, RedisConfig};
use crate::infra_db_clickhouse::service::ClickhouseService;
use crate::infra_db_redis::service::RedisService;
use crate::infra_inbound_grpc::service::GrpcServer;

pub async fn build_clickhouse_service(config: ClickhouseConfig) -> Arc<Mutex<ClickhouseService>> {
    let service = ClickhouseService::new(config).await;
    Arc::new(Mutex::new(service))
}

pub fn build_redis_service(config: RedisConfig) -> Arc<Mutex<RedisService>> {
    let service = RedisService::new(config);
    Arc::new(Mutex::new(service))
}

pub fn build_grpc_server(config: GrpcConfig) -> GrpcServer {
    GrpcServer::new(config)
}
