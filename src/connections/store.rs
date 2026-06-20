use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::lsof_stream::LsofStream;
use super::resource::Connection;

pub struct ConnectionStore {
    shared_connections: Arc<Mutex<Vec<Connection>>>,
}

impl Default for ConnectionStore {
    fn default() -> Self {
        Self {
            shared_connections: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl ConnectionStore {
    /**
     * This will run every 1 second to get and populate vectors
     * with latest identified connections
     */
    pub async fn watch(&self) -> Arc<Mutex<Vec<Connection>>> {
        let shared_connections = Arc::clone(&self.shared_connections);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;

                // LsofStream calls Command::output() which block the syscall where it waits until
                // lsof subprocess to exist with outputs. Inside tokio async task, we are not
                // suppose to call blocking functions directly as this may cause a thread to stuck
                // doing nothing (wasteful)
                let new_connections = tokio::task::spawn_blocking(LsofStream::get_connections).await.unwrap_or_default();
                let mut connections_mutex = shared_connections.lock().unwrap();
                *connections_mutex = new_connections;
            }
        });

        // this clone is really cheap for mutex because it will only increment reference count
        self.shared_connections.clone()
    }
}
