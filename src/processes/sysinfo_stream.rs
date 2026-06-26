use std::cmp::Reverse;
use std::collections::HashMap;

use sysinfo::System;

use crate::connections::resource::Connection;
use crate::utilities::bytes_format::BytesFormat;

use super::bandwidth_stream::PerProcessBandwidth;
use super::resource::Process;

/*
 * Stateful wrapper around sysinfo's System. Owns a single instance reused
 * across ticks so cpu_usage() can compute accurate deltas (requires at
 * least two refresh_all() calls on the same instance). On construction,
 * runs an immediate warm-up refresh. Each tick merges sysinfo process
 * data with lsof connection counts, cross-references per-PID, and sorts
 * by RAM descending. Upload/download rates come from BandwidthStream
 * separately.
 */
pub struct SysinfoStream {
    pub system: System,
}

impl SysinfoStream {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self { system }
    }

    pub fn get_processes(&mut self, connections: &[Connection], per_process_bandwidth: &PerProcessBandwidth) -> Vec<Process> {
        self.system.refresh_all();
        let mut process_connections_map: HashMap<u32, usize> = HashMap::new();

        // key value pairs of pid and their total connections
        for connection in connections {
            *process_connections_map.entry(connection.pid).or_default() += 1;
        }

        let mut processes: Vec<Process> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let total_connection = process_connections_map.get(&pid.as_u32()).copied().unwrap_or(0);
                let (upload_rate, download_rate) = per_process_bandwidth.get(&pid.as_u32()).copied().unwrap_or((0, 0));
                let cpu = process.cpu_usage();
                let ram = process.memory();

                Process {
                    process: process.name().to_string_lossy().to_string(),
                    pid: pid.as_u32(),
                    connections: total_connection,
                    upload: BytesFormat::format_bytes_per_seconds(upload_rate as f64),
                    upload_rate,
                    download: BytesFormat::format_bytes_per_seconds(download_rate as f64),
                    download_rate,
                    cpu: format!("{:.1}%", cpu),
                    cpu_percent: cpu as f64,
                    ram: BytesFormat::format_bytes(ram as f64),
                    ram_bytes: ram,
                }
            })
            .collect();
        processes.sort_by_key(|process| Reverse(process.ram_bytes));
        processes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_new_sysinfo_stream_then_constructs_without_panicking() {
        let _stream = SysinfoStream::new();
    }

    #[test]
    fn given_empty_connections_and_bandwidth_then_get_processes_returns_without_panicking() {
        let mut stream = SysinfoStream::new();
        let empty_bandwidth: HashMap<u32, (u64, u64)> = HashMap::new();
        let processes = stream.get_processes(&[], &empty_bandwidth);
        // Should return processes from the real system (at least the calling process)
        assert!(!processes.is_empty(), "expected at least one process on a running system");
        // Verify sort order: RAM descending
        if processes.len() >= 2 {
            assert!(processes[0].ram_bytes >= processes[1].ram_bytes, "processes must be sorted by RAM descending");
        }
    }
}
