use anyhow::Result;
use clickhouse::Client;

use crate::config::ClickhouseConfig;

pub struct ClickhouseService {
    client: Client,
}

impl ClickhouseService {
    pub async fn new(config: ClickhouseConfig) -> Self {
        let service = Self {
            client: Client::default()
                .with_url(config.address)
                .with_user(config.user)
                .with_password(config.password)
                .with_database(config.database),
        };

        service.check_connection().await
            .unwrap_or_else(|e| panic!("Cannot connect to Clickhouse: {e:?}"));

        service
    }

    pub async fn check_connection(&self) -> Result<()> {
        self.client.query("SELECT 1").execute().await?;
        tracing::debug!("Clickhouse successfully connected");
        Ok(())
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
