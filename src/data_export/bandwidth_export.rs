use crate::bandwidth::resource::BandwidthStatistic;
use crate::bandwidth::store::BandwidthStore;

use super::exporter::Exporter;

pub struct BandwidthExport;

impl Exporter for BandwidthExport {
    type Row = BandwidthStatistic;

    async fn store_and_watch(&self) -> std::sync::Arc<std::sync::Mutex<Vec<Self::Row>>> {
        BandwidthStore::default().watch().await
    }

    fn csv_header() -> &'static str {
        "name,address,mtu,upload,upload_rate,download,download_rate,total\n"
    }

    fn csv_row(row: &Self::Row) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            row.name, row.address, row.maximum_transmission_unit, row.upload, row.upload_rate, row.download, row.download_rate, row.total
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bandwidth::resource::BandwidthStatistic;

    #[test]
    fn csv_header_is_correct() {
        assert_eq!(BandwidthExport::csv_header(), "name,address,mtu,upload,upload_rate,download,download_rate,total\n");
    }

    #[test]
    fn given_bandwidth_statistic_row_then_csv_row_has_eight_fields() {
        let b = BandwidthStatistic {
            name: "en0".into(),
            address: "a4:5e:60:xx:xx:xx".into(),
            maximum_transmission_unit: "1500 B".into(),
            upload: "1.2 MB/s".into(),
            upload_rate: 1_200_000,
            download: "5.4 MB/s".into(),
            download_rate: 5_400_000,
            total: "6.6 MB/s".into(),
        };
        let row = BandwidthExport::csv_row(&b);
        let fields: Vec<&str> = row.split(',').collect();
        assert_eq!(fields.len(), 8, "csv row must match header column count");
    }

    #[test]
    fn given_bandwidth_statistic_row_then_csv_row_exact_output_matches() {
        let b = BandwidthStatistic {
            name: "en0".into(),
            address: "a4:5e:60:xx:xx:xx".into(),
            maximum_transmission_unit: "1500 B".into(),
            upload: "1.2 MB/s".into(),
            upload_rate: 1_200_000,
            download: "5.4 MB/s".into(),
            download_rate: 5_400_000,
            total: "6.6 MB/s".into(),
        };
        assert_eq!(BandwidthExport::csv_row(&b), "en0,a4:5e:60:xx:xx:xx,1500 B,1.2 MB/s,1200000,5.4 MB/s,5400000,6.6 MB/s");
    }
}
