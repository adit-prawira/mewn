use crate::connections::resource::Connection;
use crate::connections::store::ConnectionStore;

use super::exporter::Exporter;

pub struct ConnectionsExport;

impl Exporter for ConnectionsExport {
    type Row = Connection;

    async fn store_and_watch(&self) -> std::sync::Arc<std::sync::Mutex<Vec<Self::Row>>> {
        ConnectionStore::default().watch().await
    }

    fn csv_header() -> &'static str {
        "pid,process,local,remote,state,protocol\n"
    }

    fn csv_row(row: &Self::Row) -> String {
        format!("{},{},{},{},{},{}", row.pid, row.process, row.local, row.remote, row.state, row.protocol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connections::resource::Connection;

    #[test]
    fn csv_header_is_correct() {
        assert_eq!(ConnectionsExport::csv_header(), "pid,process,local,remote,state,protocol\n");
    }

    #[test]
    fn given_connection_row_then_csv_row_has_six_fields() {
        let c = Connection {
            pid: 42,
            process: "chrome".into(),
            local: "127.0.0.1:8080".into(),
            remote: "142.250.80.46:443".into(),
            state: "ESTABLISHED".into(),
            protocol: "TCP".into(),
        };
        let row = ConnectionsExport::csv_row(&c);
        let fields: Vec<&str> = row.split(',').collect();
        assert_eq!(fields.len(), 6, "csv row must match header column count");
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
        };
        assert_eq!(ConnectionsExport::csv_row(&c), "42,chrome,127.0.0.1:8080,142.250.80.46:443,ESTABLISHED,TCP");
    }
}
