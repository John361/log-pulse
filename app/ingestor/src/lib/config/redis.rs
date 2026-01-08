use serde::Deserialize;

#[derive(Deserialize)]
pub struct RedisConfig {
    address: String,
    user: String,
    password: String, // TODO: use as a secret
    pub batch_capacity: usize,
}

impl RedisConfig {
    pub fn uri(&self) -> String {
        format!("redis://{}:{}@{}", self.user, self.password, self.address)
    }
}
