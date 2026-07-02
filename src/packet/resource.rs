use std::fmt::Display;

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Packet {
    pub timestamp: String,
    pub protocol: String,
    pub source: String,
    pub destination: String,
    pub size: String,
    pub source_port: u16,
    pub destination_port: u16,
    pub raw_size: u64,
    pub dns_domain: Option<String>,
}

#[derive(Clone, PartialEq, Copy)]
pub enum ProtocolFilter {
    Tcp,
    Udp,
    Icmp,
}

impl Display for ProtocolFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolFilter::Tcp => write!(f, "TCP"),
            ProtocolFilter::Udp => write!(f, "UDP"),
            ProtocolFilter::Icmp => write!(f, "ICMP"),
        }
    }
}

impl ProtocolFilter {
    pub fn matches(&self, protocol: &str) -> bool {
        protocol.to_lowercase().starts_with(&self.to_string().to_lowercase())
    }
}
