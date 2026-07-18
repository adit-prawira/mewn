use crate::bandwidth::resource::BandwidthStatistic;
use crate::bandwidth::store::BandwidthStore;

use super::exporter::Exporter;

pub struct BandwidthExport;

impl Exporter for BandwidthExport {
    type Row = BandwidthStatistic;

    async fn store_and_watch(&self) -> std::sync::Arc<std::sync::Mutex<Vec<Self::Row>>> {
        BandwidthStore::default().watch().await
    }

    fn csv_headers() -> Vec<&'static str> {
        vec!["name", "address", "mtu", "upload", "upload_rate", "download", "download_rate", "total"]
    }

    fn csv_row_fields(row: &Self::Row) -> Vec<String> {
        vec![
            row.name.to_string(),
            row.address.to_string(),
            row.maximum_transmission_unit.to_string(),
            row.upload.to_string(),
            row.upload_rate.to_string(),
            row.download.to_string(),
            row.download_rate.to_string(),
            row.total.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bandwidth::resource::BandwidthStatistic;

    #[test]
    fn csv_header_is_correct() {
        assert_eq!(
            BandwidthExport::csv_headers(),
            vec!["name", "address", "mtu", "upload", "upload_rate", "download", "download_rate", "total"]
        );
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
        let fields = BandwidthExport::csv_row_fields(&b);
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
        assert_eq!(
            BandwidthExport::csv_row_fields(&b),
            vec!["en0", "a4:5e:60:xx:xx:xx", "1500 B", "1.2 MB/s", "1200000", "5.4 MB/s", "5400000", "6.6 MB/s"]
        );
    }

    #[test]
    fn given_name_field_with_comma_then_csv_writer_quotes_field() {
        let b = BandwidthStatistic {
            name: "en0,alias".into(),
            address: "a4:5e:60:xx:xx:xx".into(),
            maximum_transmission_unit: "1500 B".into(),
            upload: "1.2 MB/s".into(),
            upload_rate: 1_200_000,
            download: "5.4 MB/s".into(),
            download_rate: 5_400_000,
            total: "6.6 MB/s".into(),
        };
        let fields = BandwidthExport::csv_row_fields(&b);
        assert_eq!(fields[0], "en0,alias");
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer.write_record(&fields).unwrap();
        let output = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        assert_eq!(output, "\"en0,alias\",a4:5e:60:xx:xx:xx,1500 B,1.2 MB/s,1200000,5.4 MB/s,5400000,6.6 MB/s\n");
    }
}
