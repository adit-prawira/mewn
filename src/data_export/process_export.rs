use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::{Ok, Result};
use tokio::time::interval;

use crate::config::Config;
use crate::connections::store::ConnectionStore;
use crate::packet::store::PacketStore;
use crate::processes::resource::Process;
use crate::processes::store::ProcessStore;

use super::resource::ExportFormat;

pub struct ProcessExport;

impl ProcessExport {
    pub async fn export(format: &ExportFormat, output: &Path) -> Result<()> {
        let connection_store = ConnectionStore::default();
        let packet_store = PacketStore::default();
        let process_store = ProcessStore::default();

        let shared_connections = connection_store.watch().await;
        let shared_packets = packet_store.watch().await;
        let shared_processes = process_store.watch(shared_connections, shared_packets).await;

        let mut interval = interval(Duration::from_secs(Config::load().poll_interval));
        interval.tick().await;
        interval.tick().await;

        let guard = shared_processes.lock().unwrap();

        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(&*guard)?;
                fs::write(output, &json)?;
            }
            ExportFormat::Csv => {
                let mut writer = csv::Writer::from_writer(Vec::new());
                let _ = writer.write_record(Self::csv_headers());
                for datum in guard.iter() {
                    let _ = writer.write_record(Self::csv_row_fields(datum));
                }
                fs::write(output.with_extension("csv"), writer.into_inner().unwrap())?;
            }
        }
        Ok(())
    }

    pub(crate) fn csv_headers() -> Vec<&'static str> {
        vec!["process", "pid", "connections", "upload", "download", "cpu", "ram"]
    }

    pub(crate) fn csv_row_fields(row: &Process) -> Vec<String> {
        vec![
            row.process.to_string(),
            row.pid.to_string(),
            row.connections.to_string(),
            row.upload.to_string(),
            row.download.to_string(),
            row.cpu.to_string(),
            row.ram.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_header_is_correct() {
        assert_eq!(ProcessExport::csv_headers(), vec!["process", "pid", "connections", "upload", "download", "cpu", "ram"]);
    }

    #[test]
    fn given_process_row_then_csv_row_has_seven_fields() {
        let p = Process {
            process: "chrome".into(),
            pid: 1234,
            connections: 5,
            upload: "1.2 MB/s".into(),
            upload_rate: 1_200_000,
            download: "5.4 MB/s".into(),
            download_rate: 5_400_000,
            cpu: "12.5%".into(),
            cpu_percent: 12.5,
            ram: "256 MB".into(),
            ram_bytes: 268_435_456,
        };
        let fields = ProcessExport::csv_row_fields(&p);
        assert_eq!(fields.len(), 7, "csv row must match header column count");
    }

    #[test]
    fn given_process_row_then_csv_row_exact_output_matches() {
        let p = Process {
            process: "chrome".into(),
            pid: 1234,
            connections: 5,
            upload: "1.2 MB/s".into(),
            upload_rate: 1_200_000,
            download: "5.4 MB/s".into(),
            download_rate: 5_400_000,
            cpu: "12.5%".into(),
            cpu_percent: 12.5,
            ram: "256 MB".into(),
            ram_bytes: 268_435_456,
        };
        assert_eq!(ProcessExport::csv_row_fields(&p), vec!["chrome", "1234", "5", "1.2 MB/s", "5.4 MB/s", "12.5%", "256 MB"]);
    }

    #[test]
    fn given_process_name_with_comma_then_csv_writer_quotes_field() {
        let p = Process {
            process: "My App, Inc.".into(),
            pid: 1234,
            connections: 5,
            upload: "1.2 MB/s".into(),
            upload_rate: 1_200_000,
            download: "5.4 MB/s".into(),
            download_rate: 5_400_000,
            cpu: "12.5%".into(),
            cpu_percent: 12.5,
            ram: "256 MB".into(),
            ram_bytes: 268_435_456,
        };
        let fields = ProcessExport::csv_row_fields(&p);
        assert_eq!(fields[0], "My App, Inc.");
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer.write_record(&fields).unwrap();
        let output = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(output, "\"My App, Inc.\",1234,5,1.2 MB/s,5.4 MB/s,12.5%,256 MB\n");
    }

    #[test]
    fn given_process_struct_then_serializes_to_valid_json() {
        let p = Process {
            process: "spotify".into(),
            pid: 5678,
            connections: 2,
            upload: "512 KB/s".into(),
            upload_rate: 524_288,
            download: "3.1 MB/s".into(),
            download_rate: 3_250_000,
            cpu: "3.2%".into(),
            cpu_percent: 3.2,
            ram: "180 MB".into(),
            ram_bytes: 188_743_680,
        };
        let json = serde_json::to_string(&p).unwrap();
        let deserialized: Process = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.process, "spotify");
        assert_eq!(deserialized.pid, 5678);
        assert_eq!(deserialized.connections, 2);
        assert_eq!(deserialized.upload, "512 KB/s");
        assert_eq!(deserialized.upload_rate, 524_288);
        assert_eq!(deserialized.download, "3.1 MB/s");
        assert_eq!(deserialized.download_rate, 3_250_000);
        assert_eq!(deserialized.cpu, "3.2%");
        assert_eq!(deserialized.cpu_percent, 3.2);
        assert_eq!(deserialized.ram, "180 MB");
        assert_eq!(deserialized.ram_bytes, 188_743_680);
    }
}
