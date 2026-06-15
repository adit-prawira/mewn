use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::resource::Packet;
use super::stream::PacketStream;


pub struct PacketStore {
    shared_packets: Arc<Mutex<Vec<Packet>>>
}

impl Default for PacketStore {
    fn default() -> Self {
        Self { 
            shared_packets: Arc::new(Mutex::new(Vec::new()))
        }
    }
} 

impl PacketStore {
    pub async fn watch(&self) -> Arc<Mutex<Vec<Packet>>> {
        let shared_packets = Arc::clone(&self.shared_packets);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let new_packets = tokio::task::spawn_blocking(PacketStream::get_packets)
                    .await
                    .unwrap_or_default();
                let mut packets_mutex = shared_packets.lock().unwrap();
                *packets_mutex = new_packets;
            }
        });

        self.shared_packets.clone()
    }
}
