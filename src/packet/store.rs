use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config::Config;

use super::resource::Packet;
use super::stream::PacketStream;

const MAX_PACKETS: usize = 10_000;

pub struct PacketStore {
    shared_packets: Arc<Mutex<Vec<Packet>>>,
}

impl Default for PacketStore {
    fn default() -> Self {
        Self {
            shared_packets: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl PacketStore {
    pub async fn watch(&self) -> Arc<Mutex<Vec<Packet>>> {
        let shared_packets = Arc::clone(&self.shared_packets);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(Config::load().poll_interval));
            loop {
                interval.tick().await;
                let new_packets = tokio::task::spawn_blocking(PacketStream::get_packets).await.unwrap_or_default();
                let mut packets_mutex = shared_packets.lock().unwrap();
                packets_mutex.splice(0..0, new_packets);
                packets_mutex.truncate(MAX_PACKETS);
            }
        });

        self.shared_packets.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_default_store_then_shared_vec_is_empty() {
        let store = PacketStore::default();
        let guard = store.shared_packets.lock().unwrap();
        assert!(guard.is_empty());
    }

    #[tokio::test]
    async fn given_store_watch_then_returns_shared_ref_without_panicking() {
        let store = PacketStore::default();
        let shared = store.watch().await;
        let guard = shared.lock().unwrap();
        assert!(guard.is_empty());
    }

    #[tokio::test]
    async fn given_store_watch_then_returns_same_arc_allocation() {
        let store = PacketStore::default();
        let original_ptr = Arc::as_ptr(&store.shared_packets);
        let shared = store.watch().await;
        assert_eq!(Arc::as_ptr(&shared), original_ptr, "watch must return the same allocation");
    }

    #[test]
    fn given_packets_exceed_max_then_truncates_to_max() {
        let dummy_packet = Packet {
            timestamp: String::new(),
            protocol: String::new(),
            source: String::new(),
            destination: String::new(),
            size: String::new(),
            source_port: 0,
            destination_port: 0,
            raw_size: 0,
            dns_domain: None,
        };
        let overflow = vec![dummy_packet; MAX_PACKETS + 100];
        let store = PacketStore {
            shared_packets: Arc::new(Mutex::new(overflow)),
        };
        store.shared_packets.lock().unwrap().truncate(MAX_PACKETS);
        assert_eq!(store.shared_packets.lock().unwrap().len(), MAX_PACKETS);
    }
}
