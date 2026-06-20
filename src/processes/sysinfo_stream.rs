use std::cmp::Reverse;
use std::collections::HashMap;

use sysinfo::System;

use crate::connections::resource::Connection;
use crate::utilities::bytes_format::BytesFormat;

use super::resource::Process;

pub struct SysinfoStream {
    pub system: System
} 

impl SysinfoStream {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system
        }
    }

    pub fn get_processes(&mut self, connections: &[Connection]) -> Vec<Process> {
        self.system.refresh_all();
        let mut process_connections_map: HashMap<u32, usize> = HashMap::new();
        
        // key value pairs of pid and their total connections
        for connection in connections {
            *process_connections_map.entry(connection.pid).or_default() += 1;
        } 

        let mut processes: Vec<Process> = self.system.processes()
            .iter()
            .map(|(pid, process)| {
                let total_connection = process_connections_map.get(&pid.as_u32()).copied().unwrap_or(0);
                let cpu = process.cpu_usage();
                let ram = process.memory();
                
                Process {
                    process: process.name().to_string_lossy().to_string(),
                    pid: pid.as_u32(),
                    connections: total_connection,
                    upload: "-".into(),
                    upload_rate: 0,
                    download: "-".into(),
                    download_rate: 0,
                    cpu: format!("{:.1}%", cpu),
                    cpu_percent: cpu as f64,
                    ram: BytesFormat::format_bytes(ram as f64),
                    ram_bytes: ram,
                }               
            }).collect();
        processes.sort_by_key(|process| Reverse(process.ram_bytes));
        processes
    }     
}
