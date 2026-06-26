use std::path::{Path, PathBuf};

use anyhow::Result;

use super::bandwidth_export::BandwidthExport;
use super::connection_export::ConnectionsExport;
use super::exporter::Exporter;
use super::packet_export::PacketExport;
use super::process_export::ProcessExport;
use super::resource::ExportFormat;

pub struct ExportService {
    format: ExportFormat,
    output: PathBuf,
}

impl ExportService {
    pub fn new(format: ExportFormat, output: &Path) -> Self {
        Self {
            format,
            output: output.to_path_buf(),
        }
    }

    pub async fn connections(&self) -> Result<()> {
        ConnectionsExport.export(&self.format, &self.output).await
    }

    pub async fn bandwidth(&self) -> Result<()> {
        BandwidthExport.export(&self.format, &self.output).await
    }

    pub async fn packet(&self) -> Result<()> {
        PacketExport.export(&self.format, &self.output).await
    }

    pub async fn process(&self) -> Result<()> {
        ProcessExport::export(&self.format, &self.output).await
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn given_json_format_and_output_path_then_service_holds_format_and_path() {
        let service = ExportService::new(ExportFormat::Json, Path::new("/tmp/test.json"));
        assert!(matches!(service.format, ExportFormat::Json));
        assert_eq!(service.output, PathBuf::from("/tmp/test.json"));
    }

    #[test]
    fn given_csv_format_and_output_path_then_service_holds_csv_format() {
        let service = ExportService::new(ExportFormat::Csv, Path::new("/tmp/out"));
        assert!(matches!(service.format, ExportFormat::Csv));
        assert_eq!(service.output, PathBuf::from("/tmp/out"));
    }
}
