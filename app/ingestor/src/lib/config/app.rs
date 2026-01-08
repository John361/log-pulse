use serde::Deserialize;

use crate::config::{ClickhouseConfig, GrpcConfig, RedisConfig, WorkerConfig};

#[derive(Deserialize)]
pub struct AppConfig {
    pub clickhouse: ClickhouseConfig,
    pub grpc: GrpcConfig,
    pub redis: RedisConfig,
    pub worker: WorkerConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new("./app.conf.yml")
    }
}

impl AppConfig {
    pub fn new(path: &str) -> Self {
        let config = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .unwrap_or_else(|e| panic!("Cannot get app config file path: {e:?}"));

        config
            .try_deserialize::<AppConfig>()
            .unwrap_or_else(|e| panic!("Cannot deserialize app config: {e:?}"))
    }
}
