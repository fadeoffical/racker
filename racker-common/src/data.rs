use std::net::SocketAddr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Network {
    host: String,
    port: u16,
}

impl Network {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }
}

impl Network {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn as_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host.parse().unwrap(), self.port)
    }
}

impl From<SocketAddr> for Network {
    fn from(value: SocketAddr) -> Self {
        Self {
            host: value.ip().to_string(),
            port: value.port(),
        }
    }
}
