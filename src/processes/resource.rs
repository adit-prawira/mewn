use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Process {
    pub process: String,
    pub pid: u32,
    pub connections: usize,
    pub upload: String,
    pub upload_rate: u64,
    pub download: String,
    pub download_rate: u64,
    pub cpu: String,
    pub cpu_percent: f64,
    pub ram: String,
    pub ram_bytes: u64,
}
