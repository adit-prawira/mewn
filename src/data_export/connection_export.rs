use crate::connections::resource::Connection;
use crate::connections::store::ConnectionStore;

use super::exporter::Exporter;

pub struct ConnectionsExport;

impl Exporter for ConnectionsExport {
    type Row = Connection;

    async fn store_and_watch(&self) -> std::sync::Arc<std::sync::Mutex<Vec<Self::Row>>> {
        ConnectionStore::default().watch().await
    }

    fn csv_headers() -> Vec<&'static str> {
        vec!["pid", "process", "local", "remote", "state", "protocol", "country"]
    }

    fn csv_row_fields(row: &Self::Row) -> Vec<String> {
        let country = row.country.as_deref().unwrap_or("-");
        vec![
            row.pid.to_string(),
            row.process.to_string(),
            row.local.to_string(),
            row.remote.to_string(),
            row.state.to_string(),
            row.protocol.to_string(),
            country.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connections::resource::Connection;

    #[test]
    fn csv_header_is_correct() {
        assert_eq!(ConnectionsExport::csv_headers(), vec!["pid", "process", "local", "remote", "state", "protocol", "country"]);
    }

    #[test]
    fn given_connection_row_then_csv_row_has_seven_fields() {
        let c = Connection {
            pid: 42,
            process: "chrome".into(),
            local: "127.0.0.1:8080".into(),
            remote: "142.250.80.46:443".into(),
            state: "ESTABLISHED".into(),
            protocol: "TCP".into(),
            country: None,
        };
        let fields = ConnectionsExport::csv_row_fields(&c);
        assert_eq!(fields.len(), 7, "csv row must match header column count");
    }

    #[test]
    fn given_connection_row_then_csv_row_exact_output_matches() {
        let c = Connection {
            pid: 42,
            process: "chrome".into(),
            local: "127.0.0.1:8080".into(),
            remote: "142.250.80.46:443".into(),
            state: "ESTABLISHED".into(),
            protocol: "TCP".into(),
            country: Some("US".into()),
        };
        assert_eq!(
            ConnectionsExport::csv_row_fields(&c),
            vec!["42", "chrome", "127.0.0.1:8080", "142.250.80.46:443", "ESTABLISHED", "TCP", "US"]
        );
    }

    #[test]
    fn given_process_field_with_comma_then_csv_writer_quotes_field() {
        let c = Connection {
            pid: 42,
            process: "My App, Inc.".into(),
            local: "127.0.0.1:8080".into(),
            remote: "142.250.80.46:443".into(),
            state: "ESTABLISHED".into(),
            protocol: "TCP".into(),
            country: Some("US".into()),
        };
        let fields = ConnectionsExport::csv_row_fields(&c);
        assert_eq!(fields[1], "My App, Inc.");
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer.write_record(&fields).unwrap();
        let output = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(output, "42,\"My App, Inc.\",127.0.0.1:8080,142.250.80.46:443,ESTABLISHED,TCP,US\n");
    }
}
