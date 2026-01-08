use serde::Deserialize;

use crate::config::Secret;

#[derive(Deserialize)]
pub struct ClickhouseConfig {
    pub address: String,
    pub user: String,
    pub password: Secret,
    pub database: String,
}
