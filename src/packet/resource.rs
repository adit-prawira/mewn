#[derive(Clone)]
pub struct Packet {
    pub timestamp: String,
    pub protocol: String,
    pub source: String,
    pub destination: String,
    pub size: String,
    pub source_port: u16,
    pub destination_port: u16,
    pub raw_size: u64,
    pub dns_domain: Option<String>
}
