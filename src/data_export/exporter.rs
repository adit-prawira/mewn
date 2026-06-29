use std::fs;
use std::future::Future;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Ok, Result};
use serde::Serialize;
use tokio::time::interval;

use super::resource::ExportFormat;

pub trait Exporter {
    type Row: Serialize + Send;

    fn store_and_watch(&self) -> impl Future<Output = Arc<Mutex<Vec<Self::Row>>>> + Send;

    fn csv_header() -> &'static str;
    fn csv_row(row: &Self::Row) -> String;

    fn export(&self, format: &ExportFormat, output: &Path) -> impl Future<Output = Result<()>> + Send
    where
        Self: Sync,
    {
        async move {
            let shared = self.store_and_watch().await;
            let mut interval = interval(Duration::from_secs(1));
            interval.tick().await;
            interval.tick().await;

            let guard = shared.lock().unwrap();
            match format {
                ExportFormat::Json => {
                    let json = serde_json::to_string_pretty(&*guard)?;
                    fs::write(output, &json)?;
                }
                ExportFormat::Csv => {
                    let mut csv = String::from(Self::csv_header());
                    for row in guard.iter() {
                        csv.push_str(&Self::csv_row(row));
                        csv.push('\n');
                    }
                    fs::write(output.with_extension("csv"), &csv)?;
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::{Arc, Mutex};

    use serde::{Deserialize, Serialize};
    use tempfile::tempdir;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestRow {
        name: String,
        count: u32,
    }

    struct TestExporter {
        data: Arc<Mutex<Vec<TestRow>>>,
    }

    impl TestExporter {
        fn new(data: Vec<TestRow>) -> Self {
            Self { data: Arc::new(Mutex::new(data)) }
        }
    }

    impl Exporter for TestExporter {
        type Row = TestRow;

        async fn store_and_watch(&self) -> Arc<Mutex<Vec<TestRow>>> {
            Arc::clone(&self.data)
        }

        fn csv_header() -> &'static str {
            "name,count\n"
        }

        fn csv_row(row: &TestRow) -> String {
            format!("{},{}", row.name, row.count)
        }
    }

    #[tokio::test]
    async fn given_data_and_json_format_then_file_contains_expected_json() {
        let dir = tempdir().unwrap();
        let output = dir.path().join("test.json");
        let exporter = TestExporter::new(vec![TestRow { name: "foo".into(), count: 1 }, TestRow { name: "bar".into(), count: 2 }]);

        exporter.export(&ExportFormat::Json, &output).await.unwrap();

        let content = fs::read_to_string(&output).unwrap();
        let deserialized: Vec<TestRow> = serde_json::from_str(&content).unwrap();
        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized[0], TestRow { name: "foo".into(), count: 1 });
        assert_eq!(deserialized[1], TestRow { name: "bar".into(), count: 2 });
    }

    #[tokio::test]
    async fn given_data_and_csv_format_then_file_contains_header_and_rows() {
        let dir = tempdir().unwrap();
        let output = dir.path().join("test");
        let exporter = TestExporter::new(vec![TestRow { name: "foo".into(), count: 1 }, TestRow { name: "bar".into(), count: 2 }]);

        exporter.export(&ExportFormat::Csv, &output).await.unwrap();

        let csv_path = output.with_extension("csv");
        let content = fs::read_to_string(&csv_path).unwrap();
        let lines: Vec<&str> = content.trim().lines().collect();
        assert_eq!(lines.len(), 3, "expected header + 2 data rows");
        assert_eq!(lines[0], "name,count");
        assert_eq!(lines[1], "foo,1");
        assert_eq!(lines[2], "bar,2");
    }

    #[tokio::test]
    async fn given_empty_store_and_csv_format_then_file_contains_header_only() {
        let dir = tempdir().unwrap();
        let output = dir.path().join("test");
        let exporter = TestExporter::new(vec![]);

        exporter.export(&ExportFormat::Csv, &output).await.unwrap();

        let csv_path = output.with_extension("csv");
        let content = fs::read_to_string(&csv_path).unwrap();
        assert_eq!(content, "name,count\n");
    }

    #[tokio::test]
    async fn given_empty_store_and_json_format_then_file_contains_empty_array() {
        let dir = tempdir().unwrap();
        let output = dir.path().join("test.json");
        let exporter = TestExporter::new(vec![]);

        exporter.export(&ExportFormat::Json, &output).await.unwrap();

        let content = fs::read_to_string(&output).unwrap();
        assert_eq!(content, "[]");
    }
}
