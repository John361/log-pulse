use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct GrpcConfig {
    address: String,
    port: u16,
}

impl GrpcConfig {
    pub fn uri(&self) -> SocketAddr {
        format!("{}:{}", self.address, self.port)
            .parse()
            .unwrap_or_else(|e| panic!("Cannot parse grpc config uri: {e:?}"))
    }
}
