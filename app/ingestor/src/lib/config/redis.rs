use serde::Deserialize;

use crate::config::Secret;

#[derive(Deserialize)]
pub struct RedisConfig {
    address: String,
    user: String,
    password: Secret,
    pub batch_capacity: usize,
}

impl RedisConfig {
    pub fn uri(&self) -> String {
        format!("redis://{}:{}@{}", self.user, self.password.expose(), self.address)
    }
}
