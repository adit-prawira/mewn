use std::collections::HashMap;
use std::process::Command;
use std::sync::Mutex;
use std::time::Instant;

use crate::bandwidth::resource::BandwidthStatistic;
use crate::utilities::bytes_format::BytesFormat;

struct NetstatEntry {
    name: String, 
    mtu: u64,
    address: String,
    ibytes: u64,
    obytes: u64
}

static LAST_NETSTAT_ENTRIES_SNAPSHOT: Mutex<Option<(Instant, HashMap<String, NetstatEntry>)>> = Mutex::new(None);

pub struct NetstatStream;

/*
 * NetstatStream executes netstat -ib and parses the output into a list of active
 * network bandwidth.
 *
 * This will conver each nestat entry into BandwidthStatistic object and return vector of it.
 * */
impl NetstatStream { 
    pub fn get_statistics() -> Vec<BandwidthStatistic> {
        let output = Command::new("netstat")
            .args(["-ib"])
            .output();

        let Ok(results) = output else {return Vec::new();};
        if !results.status.success() {return Vec::new();};

        let stdout = String::from_utf8_lossy(&results.stdout);
        let netstat_entries = Self::parse_netstat_output(&stdout);
        
        let mut last_netstat_entries_snapshot = LAST_NETSTAT_ENTRIES_SNAPSHOT.lock().unwrap();
        let now = Instant::now();

        let results = if let Some((last_time, last_netstat_entries_map)) = last_netstat_entries_snapshot.as_ref() {
            let elapsed = now.duration_since(*last_time).as_secs_f64();
            let has_elapsed = elapsed > 0.0;
            if !has_elapsed {return Vec::new();};

            let mut statistics: Vec<BandwidthStatistic> = Vec::new();
            for current_entry in &netstat_entries {
                let Some(last_entry) = last_netstat_entries_map.get(&current_entry.name) else {continue;};
                let upload_rate = (current_entry.obytes.saturating_sub(last_entry.obytes) as f64) / elapsed; 
                let download_rate = (current_entry.ibytes.saturating_sub(last_entry.ibytes) as f64) / elapsed;
                let maximum_transmission_unit = current_entry.mtu.saturating_sub(last_entry.mtu) as f64;
                let total_rate = upload_rate + download_rate;

                statistics.push(BandwidthStatistic { 
                    name: last_entry.name.to_string(),
                    address: last_entry.address.to_string(),
                    maximum_transmission_unit: BytesFormat::format_bytes(maximum_transmission_unit),
                    upload: BytesFormat::format_bytes_per_seconds(upload_rate), 
                    upload_rate: upload_rate as u64,
                    download: BytesFormat::format_bytes_per_seconds(download_rate),
                    download_rate: download_rate as u64,
                    total: BytesFormat::format_bytes_per_seconds(total_rate) 
                });
            }
            statistics
        } else {
            let statistics: Vec<BandwidthStatistic> = netstat_entries.iter()
                .map(|entry| BandwidthStatistic { 
                    name: entry.name.to_string(), 
                    address: entry.address.to_string(), 
                    maximum_transmission_unit: BytesFormat::format_bytes(entry.mtu as f64), 
                    upload: BytesFormat::format_bytes_per_seconds(0.0),
                    upload_rate: 0,
                    download: BytesFormat::format_bytes_per_seconds(0.0), 
                    download_rate: 0,
                    total: BytesFormat::format_bytes_per_seconds(0.0) 
                })
                .collect();
            statistics
        };

        let netstat_entries_map: HashMap<String, NetstatEntry> = netstat_entries.into_iter()
            .map(|entry| (entry.name.to_string(), entry))
            .collect();
        *last_netstat_entries_snapshot = Some((now, netstat_entries_map));

        results 
    }

    fn parse_netstat_output(output: &str) -> Vec<NetstatEntry> {
        let mut lines = output.lines(); 
        
        let Some(header) = lines.next() else {return Vec::new();};
        let header_parts: Vec<&str> = header.split_whitespace().collect();

        let total_bandwidth_parts = header_parts.len();

        let mut entries = Vec::new();
        for line in lines {
            let Some(entry) = Self::parse_line(line, total_bandwidth_parts) else {continue;};
            entries.push(entry);
        }

        entries 
    }

    /*
     * Name       Mtu   Network       Address            Ipkts Ierrs     Ibytes    Opkts Oerrs     Obytes  Coll
     * lo0        yoooo <Link#1>                         20762     0    3269247    20762     0    3269247     0
     * lo0        yoooo 127           localhost          20762     -    3269247    20762     -    3269247     -
     *
     * this is because when address is empty there will be 10 parts of the entry when split
     * with whitepace instead of 11 and thus access the right data may require to shift parts index
     * */
    fn parse_line(line: &str, expected_total_bandwidth_parts: usize) -> Option<NetstatEntry> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let minimum_total_bandwidth_parts = expected_total_bandwidth_parts - 1;
        if parts.len() < minimum_total_bandwidth_parts {
            return None;
        }

        let has_address = parts.len() > minimum_total_bandwidth_parts;
        let (ibytes_index, obytes_index) = if has_address {
            (6, 9)
        } else {
            (5, 8)
        };

        let network = parts[2]; 
        if !network.starts_with("<Link#") {return None;};
        let name = parts[0].to_string();
        let mtu = parts[1].parse::<u64>().ok()?;
        let address = if has_address {
            parts[3].to_string()
        } else {
            String::from("N/A")
        };
        let ibytes = parts[ibytes_index].parse::<u64>().ok()?;
        let obytes = parts[obytes_index].parse::<u64>().ok()?;
        
        Some(NetstatEntry { name, mtu, address, ibytes, obytes })
    }  
}
