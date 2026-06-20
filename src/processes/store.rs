use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::connections::resource::Connection;
use crate::packet::resource::Packet;

use super::bandwidth_stream::BandwidthStream;
use super::resource::Process;
use super::sysinfo_stream::{SysinfoStream};

pub struct ProcessStore {
    shared_processes: Arc<Mutex<Vec<Process>>>,
}

impl Default for ProcessStore {
    fn default() -> Self {
        Self { 
            shared_processes: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl ProcessStore {
    pub async fn watch(&self, shared_connections: Arc<Mutex<Vec<Connection>>>, shared_packets: Arc<Mutex<Vec<Packet>>>) -> Arc<Mutex<Vec<Process>>> {
        let shared_process = Arc::clone(&self.shared_processes);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            let mut sysinfo = SysinfoStream::new();

            loop {
                interval.tick().await;
                let connections_snapshot = {
                    let guard = shared_connections.lock().unwrap();
                    guard.clone()
                };
                let packets_snapshot = {
                    let guard = shared_packets.lock().unwrap();
                    guard.clone()
                };
                let per_process_bandwidth = BandwidthStream::compute(&connections_snapshot, &packets_snapshot);
                let new_processes = sysinfo.get_processes(&connections_snapshot, &per_process_bandwidth);
                let mut processes_mutex = shared_process.lock().unwrap();
                *processes_mutex = new_processes;
            }
        });
        self.shared_processes.clone()
    }
}


