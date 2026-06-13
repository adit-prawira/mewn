use crate::bandwidth::resource::BandwidthStatistic;

pub struct BandwidthStream;

impl BandwidthStream {
    pub fn get_statistics() -> Vec<BandwidthStatistic> {
        Self::mock_data()
    }

    fn mock_data() -> Vec<BandwidthStatistic> {
        vec![
            BandwidthStatistic {
               interface: "en0".to_string(),
               upload: "1.2 MB/s".to_string(),
               download: "5.4 MB/s".to_string(),
               total: "6.6 MB/s".to_string(),
            },
            BandwidthStatistic {
               interface: "lo0".to_string(),
               upload: "0.1 MB/s".to_string(),
               download: "0.1 MB/s".to_string(),
               total: "0.2 MB/s".to_string(),
            },
        ]
    }
}
