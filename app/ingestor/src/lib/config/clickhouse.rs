use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClickhouseConfig {
    pub address: String,
    pub user: String,
    pub password: String, // TODO: use as a secret
    pub database: String,
}
