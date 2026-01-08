use redis::{Client, ConnectionLike};
use redis::aio::MultiplexedConnection;

use crate::config::RedisConfig;

pub struct RedisService {
    connection: MultiplexedConnection,
    pub database: String,
    pub batch_capacity: usize,
}

impl RedisService {
    pub async fn new(config: RedisConfig) -> Self {
        Self {
            connection: Self::connect(&config).await,
            database: config.database,
            batch_capacity: config.batch_capacity,
        }
    }

    async fn connect(config: &RedisConfig) -> MultiplexedConnection {
        let client = Client::open(config.uri())
            .unwrap_or_else(|e| panic!("Cannot get redis client connection: {e:?}"));

        let connected = client
            .get_connection()
            .expect("Failed to connect to Redis (Invalid credentials or server down)")
            .check_connection();

        if connected {
            client.get_multiplexed_async_connection().await
                .inspect(|_| tracing::debug!("Redis connection established"))
                .unwrap_or_else(|e| panic!("Cannot get redis client connection: {e:?}"))
        } else {
            panic!("Failed to connect to Redis (Invalid credentials or server down)");
        }
    }

    pub fn connection(&self) -> &MultiplexedConnection {
        &self.connection
    }
}
