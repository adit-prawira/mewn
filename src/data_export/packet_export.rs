use crate::packet::resource::Packet;
use crate::packet::store::PacketStore;

use super::exporter::Exporter;

pub struct PacketExport;

impl Exporter for PacketExport {
    type Row = Packet;

    async fn store_and_watch(&self) -> std::sync::Arc<std::sync::Mutex<Vec<Self::Row>>> {
        PacketStore::default().watch().await
    }

    fn csv_header() -> &'static str {
        "timestamp,protocol,source,source_port,destination,destination_port,size\n"
    }

    fn csv_row(row: &Self::Row) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            row.timestamp, row.protocol, row.source, row.source_port, row.destination, row.destination_port, row.size
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::resource::Packet;

    #[test]
    fn csv_header_is_correct() {
        assert_eq!(PacketExport::csv_header(), "timestamp,protocol,source,source_port,destination,destination_port,size\n");
    }

    #[test]
    fn given_packet_row_then_csv_row_has_seven_fields() {
        let p = Packet {
            timestamp: "12:34:56".into(),
            protocol: "TCP".into(),
            source: "192.168.1.5".into(),
            source_port: 52532,
            destination: "142.250.80.46".into(),
            destination_port: 443,
            size: "64 B".into(),
            raw_size: 64,
            dns_domain: None,
        };
        let row = PacketExport::csv_row(&p);
        let fields: Vec<&str> = row.split(',').collect();
        assert_eq!(fields.len(), 7, "csv row must match header column count");
    }

    #[test]
    fn given_packet_row_then_csv_row_exact_output_matches() {
        let p = Packet {
            timestamp: "12:34:56".into(),
            protocol: "TCP".into(),
            source: "192.168.1.5".into(),
            source_port: 52532,
            destination: "142.250.80.46".into(),
            destination_port: 443,
            size: "64 B".into(),
            raw_size: 64,
            dns_domain: None,
        };
        assert_eq!(PacketExport::csv_row(&p), "12:34:56,TCP,192.168.1.5,52532,142.250.80.46,443,64 B");
    }
}
