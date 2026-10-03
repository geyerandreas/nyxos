use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Local {
    pub id: IpAddr,
    pub port: u16,
}

impl Default for Local {
    fn default() -> Self {
        Self {
            id: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 8080,
        }
    }
}
