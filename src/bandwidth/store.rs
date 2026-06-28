use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config::Config;

use super::netstat_stream::NetstatStream;
use super::resource::BandwidthStatistic;

pub struct BandwidthStore {
    shared_bandwidth_statistics: Arc<Mutex<Vec<BandwidthStatistic>>>,
}

impl Default for BandwidthStore {
    fn default() -> Self {
        Self {
            shared_bandwidth_statistics: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl BandwidthStore {
    pub async fn watch(&self) -> Arc<Mutex<Vec<BandwidthStatistic>>> {
        let shared_bandwidth_statistics = Arc::clone(&self.shared_bandwidth_statistics);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(Config::load().poll_interval));
            loop {
                interval.tick().await;
                let bandwidth_statistics = tokio::task::spawn_blocking(NetstatStream::get_statistics).await.unwrap_or_default();
                let mut bandwidth_statistics_mutex = shared_bandwidth_statistics.lock().unwrap();
                *bandwidth_statistics_mutex = bandwidth_statistics;
            }
        });
        self.shared_bandwidth_statistics.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_default_store_then_shared_vec_is_empty() {
        let store = BandwidthStore::default();
        let guard = store.shared_bandwidth_statistics.lock().unwrap();
        assert!(guard.is_empty());
    }

    #[tokio::test]
    async fn given_store_watch_then_returns_shared_ref_without_panicking() {
        let store = BandwidthStore::default();
        let shared = store.watch().await;
        let guard = shared.lock().unwrap();
        assert!(guard.is_empty());
    }

    #[tokio::test]
    async fn given_store_watch_then_returns_same_arc_allocation() {
        let store = BandwidthStore::default();
        let original_ptr = Arc::as_ptr(&store.shared_bandwidth_statistics);
        let shared = store.watch().await;
        assert_eq!(Arc::as_ptr(&shared), original_ptr, "watch must return the same allocation");
    }
}
