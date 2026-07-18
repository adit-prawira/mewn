use crate::packet::resource::Packet;
use crate::packet::store::PacketStore;

use super::exporter::Exporter;

pub struct PacketExport;

impl Exporter for PacketExport {
    type Row = Packet;

    async fn store_and_watch(&self) -> std::sync::Arc<std::sync::Mutex<Vec<Self::Row>>> {
        PacketStore::default().watch().await
    }

    fn csv_headers() -> Vec<&'static str> {
        vec!["timestamp", "protocol", "source", "source_port", "destination", "destination_port", "size"]
    }

    fn csv_row_fields(row: &Self::Row) -> Vec<String> {
        vec![
            row.timestamp.to_string(),
            row.protocol.to_string(),
            row.source.to_string(),
            row.source_port.to_string(),
            row.destination.to_string(),
            row.destination_port.to_string(),
            row.size.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::resource::Packet;

    #[test]
    fn csv_header_is_correct() {
        assert_eq!(
            PacketExport::csv_headers(),
            vec!["timestamp", "protocol", "source", "source_port", "destination", "destination_port", "size"]
        );
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
        let fields = PacketExport::csv_row_fields(&p);
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
        assert_eq!(
            PacketExport::csv_row_fields(&p),
            vec!["12:34:56", "TCP", "192.168.1.5", "52532", "142.250.80.46", "443", "64 B"]
        );
    }

    #[test]
    fn given_protocol_field_with_comma_then_csv_writer_quotes_field() {
        let p = Packet {
            timestamp: "12:34:56".into(),
            protocol: "TCP,TLP".into(),
            source: "192.168.1.5".into(),
            source_port: 52532,
            destination: "142.250.80.46".into(),
            destination_port: 443,
            size: "64 B".into(),
            raw_size: 64,
            dns_domain: None,
        };
        let fields = PacketExport::csv_row_fields(&p);
        assert_eq!(fields[1], "TCP,TLP");
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer.write_record(&fields).unwrap();
        let output = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(output, "12:34:56,\"TCP,TLP\",192.168.1.5,52532,142.250.80.46,443,64 B\n");
    }
}
