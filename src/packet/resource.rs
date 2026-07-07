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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_tcp_filter_when_protocol_is_tcp_then_matches() {
        assert!(ProtocolFilter::Tcp.matches("tcp"));
    }

    #[test]
    fn given_udp_filter_when_protocol_is_udp_then_matches() {
        assert!(ProtocolFilter::Udp.matches("udp"));
    }

    #[test]
    fn given_icmp_filter_when_protocol_is_icmp_then_matches() {
        assert!(ProtocolFilter::Icmp.matches("icmp"));
    }

    #[test]
    fn given_any_filter_when_protocol_is_uppercase_then_matches() {
        assert!(ProtocolFilter::Tcp.matches("TCP"));
        assert!(ProtocolFilter::Udp.matches("UDP"));
        assert!(ProtocolFilter::Icmp.matches("ICMP"));
    }

    #[test]
    fn given_any_filter_when_protocol_is_empty_then_does_not_match() {
        assert!(!ProtocolFilter::Tcp.matches(""));
        assert!(!ProtocolFilter::Udp.matches(""));
        assert!(!ProtocolFilter::Icmp.matches(""));
    }

    #[test]
    fn given_tcp_filter_when_protocol_starts_with_tcp_segment_then_matches() {
        assert!(ProtocolFilter::Tcp.matches("tcp6"));
        assert!(ProtocolFilter::Tcp.matches("tcp4"));
    }
}
