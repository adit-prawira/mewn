use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use crate::connections::resource::Connection;
use crate::packet::resource::Packet;

pub type PerProcessBandwidth = HashMap<u32, (u64, u64)>;

static PREVIOUS_STATE: Mutex<Option<(Instant, PerProcessBandwidth)>> = Mutex::new(None);

/*
 * Computes per-process upload and download rates by cross-referencing
 * live packet capture data with lsof connection mappings. Each tick:
 *
 * 1. Builds a port→PID map from Connection.local addresses (e.g.,
 *    "192.168.1.5:52532" → PID 1234).
 * 2. Accumulates packet bytes into per-PID cumulative counters:
 *    - source_port matches local lsof port → upload (our machine sent)
 *    - destination_port matches local lsof port → download (our machine received)
 * 3. Computes bytes/sec via delta from the previous tick's cumulative
 *    snapshot: (cumulative_now − cumulative_prev) / elapsed_seconds.
 *
 * Maintains static cumulative state (never reset) so rate diffs are
 * always meaningful regardless of per-window packet count variance.
 * First call returns an empty map (no baseline). Only PIDs with
 * non-zero deltas appear in the output.
 */
pub struct BandwidthStream;

impl BandwidthStream {
    /*
     * The method will calculate upload and download rate of each process PID.
     * Thus, returning key value pairs of PID -> (upload_rate, download_rate)
     */
    pub fn compute(connections: &[Connection], packets: &[Packet]) -> PerProcessBandwidth {
        let port_pid_map = Self::build_port_pid_map(connections);
        let mut previous_state = PREVIOUS_STATE.lock().unwrap();
        let now = Instant::now();

        let (previous_time, previous_cumulative) = match previous_state.take() {
            Some(state) => state,
            None => {
                let mut cumulative: HashMap<u32, (u64, u64)> = HashMap::new();
                Self::accumulate(packets, port_pid_map, &mut cumulative);
                *previous_state = Some((now, cumulative));
                return HashMap::new();
            }
        };

        let mut new_cumulative = previous_cumulative.clone();
        Self::accumulate(packets, port_pid_map, &mut new_cumulative);

        let elapsed = now.duration_since(previous_time).as_secs_f64().max(0.1);
        let mut rates: PerProcessBandwidth = HashMap::new();

        for (pid, (upload_size, download_size)) in &new_cumulative {
            let (previous_upload_size, previous_download_size) = previous_cumulative.get(pid).copied().unwrap_or((0, 0));
            let delta_upload_size = upload_size.saturating_sub(previous_upload_size);
            let delta_download_size = download_size.saturating_sub(previous_download_size);
            if delta_upload_size > 0 || delta_download_size > 0 {
                rates.insert(*pid, ((delta_upload_size as f64 / elapsed) as u64, (delta_download_size as f64 / elapsed) as u64));
            }
        }

        *previous_state = Some((now, new_cumulative));
        rates
    }

    /*
     * Summing packets upload and download size
     */
    fn accumulate(packets: &[Packet], port_pid_map: HashMap<u16, u32>, cumulative: &mut HashMap<u32, (u64, u64)>) {
        for packet in packets {
            // Get packet size for upload from source port
            // as our machine is sending data
            if let Some(pid) = port_pid_map.get(&packet.source_port) {
                cumulative.entry(*pid).or_default().0 += packet.raw_size;
            }

            // get packet size for donwload from destination port
            // as our machine is receiving data
            if let Some(pid) = port_pid_map.get(&packet.destination_port) {
                cumulative.entry(*pid).or_default().1 += packet.raw_size;
            }
        }
    }

    fn build_port_pid_map(connections: &[Connection]) -> HashMap<u16, u32> {
        let mut map: HashMap<u16, u32> = HashMap::new();
        for connection in connections {
            let Some(port) = connection.local.rsplit(":").next().and_then(|port| port.parse::<u16>().ok()) else {
                continue;
            };
            map.insert(port, connection.pid);
        }
        map
    }
}
