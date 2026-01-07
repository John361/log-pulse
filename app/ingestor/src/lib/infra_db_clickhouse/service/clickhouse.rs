use clickhouse::Client;

use crate::config::ClickhouseConfig;

pub struct ClickhouseService {
    client: Client,
}

impl ClickhouseService {
    pub fn new(config: ClickhouseConfig) -> Self {
        Self {
            client: Client::default()
                .with_url(config.address)
                .with_user(config.user)
                .with_password(config.password)
                .with_database(config.database),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
