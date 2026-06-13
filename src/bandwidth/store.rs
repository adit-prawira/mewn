use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::resource::BandwidthStatistic;
use super::statistic_stream::BandwidthStream;

pub struct BandwidthStore {
    shared_bandwidth_statistics: Arc<Mutex<Vec<BandwidthStatistic>>>
}

impl Default for BandwidthStore {
    fn default() -> Self {
        Self { shared_bandwidth_statistics: Arc::new(Mutex::new(Vec::new())) }
    }
}

impl BandwidthStore {
    pub async fn watch(&self) -> Arc<Mutex<Vec<BandwidthStatistic>>> {
        let shared_bandwidth_statistics = Arc::clone(&self.shared_bandwidth_statistics);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let bandwidth_statistics = tokio::task::spawn_blocking(BandwidthStream::get_statistics)
                    .await
                    .unwrap_or_default();
                let mut bandwidth_statistics_mutex = shared_bandwidth_statistics.lock().unwrap();
                *bandwidth_statistics_mutex = bandwidth_statistics;
            }
        }); 
        self.shared_bandwidth_statistics.clone()
    }
}


