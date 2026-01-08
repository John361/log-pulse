use anyhow::Result;
use redis::{Client, ConnectionLike};
use redis::aio::MultiplexedConnection;

use crate::config::RedisConfig;

pub struct RedisService {
    client: Client,
}

impl RedisService {
    pub fn new(config: RedisConfig) -> Self {
        Self {
            client: Self::connect(config),
        }
    }

    fn connect(config: RedisConfig) -> Client {
        let client = Client::open(config.uri())
            .unwrap_or_else(|e| panic!("Cannot get redis client connection: {e:?}"));

        let connected = client
            .get_connection()
            .expect("Failed to connect to Redis (Invalid credentials or server down)")
            .check_connection();

        if connected {
            tracing::debug!("Redis successfully connected");
            client
        } else {
            panic!("Failed to connect to Redis (Invalid credentials or server down)");
        }
    }

    pub async fn connection(&self) -> Result<MultiplexedConnection> {
        let connection = self.client.get_multiplexed_async_connection().await?;
        Ok(connection)
    }
}
