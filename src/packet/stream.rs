use std::net::IpAddr;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use etherparse::{NetSlice, SlicedPacket, TransportSlice};
use pcap::{Active, Capture, Device};

use crate::utilities::bytes_format::BytesFormat;

use super::resource::Packet;

static CAPTURE: Mutex<Option<Capture<Active>>> = Mutex::new(None);
static CAPTURE_START: Mutex<Option<Instant>> = Mutex::new(None);

pub struct PacketStream;

impl PacketStream {
    pub fn get_packets() -> Vec<Packet> {
        let mut new_packets: Vec<Packet> = Vec::new(); 
        let mut capture_guard = CAPTURE.lock().unwrap();

        let capture = match capture_guard.as_mut() {
            Some(c) => c,
            None => {
                let Some(devices) = Device::list().ok() else { return Vec::new(); };
                let Some(device) = devices.iter()
                    .filter(|device| device.flags.is_up() 
                        && device.flags.is_running()
                        && !device.flags.is_loopback()
                        && !device.addresses.is_empty()
                    )
                    .min_by_key(|device| {
                        let has_routable_ipv4 = device.addresses.iter().any(|address| {
                            if let IpAddr::V4(ipv4) = address.addr {
                                !ipv4.is_link_local()
                            } else {
                                false
                            }
                        });
                        if has_routable_ipv4 { 0 } else { 1 }
                    }) 
                else { return Vec::new(); };

                let Ok(inactive) = Capture::from_device(device.clone()) else { return Vec::new(); };
                let Some(active) = inactive.promisc(true).snaplen(65535).timeout(100).open().ok() else { return Vec::new(); };
                *CAPTURE_START.lock().unwrap() = Some(Instant::now());
                capture_guard.insert(active)
            }
        };

        let deadline = Instant::now() + Duration::from_millis(100);
        
        while let Ok(packet) = capture.next_packet() {
            let Ok(parsed_packet) = SlicedPacket::from_ethernet(packet.data) else {continue;};
            let start = *CAPTURE_START.lock().unwrap().get_or_insert(Instant::now());
            let timestamp = format!("+{:.3}s", start.elapsed().as_secs_f64());
            
            let (protocol, source_port, destination_port) = match parsed_packet.transport {
                Some(TransportSlice::Tcp(tcp)) => ("TCP", tcp.source_port(), tcp.destination_port()),
                Some(TransportSlice::Udp(udp)) => ("UDP", udp.source_port(), udp.destination_port()),
                Some(TransportSlice::Icmpv4(_)) => ("ICMPV4", 0, 0),
                Some(TransportSlice::Icmpv6(_)) => ("ICMPV6", 0, 0),
                None => ("???", 0, 0)
            };

            let (source_ip, destination_ip) = match parsed_packet.net {
                Some(NetSlice::Ipv4(ipv4)) => (ipv4.header().source_addr().to_string(), ipv4.header().destination_addr().to_string()),
                Some(NetSlice::Ipv6(ipv6)) => (ipv6.header().source_addr().to_string(), ipv6.header().destination_addr().to_string()),
                Some(NetSlice::Arp(_)) => ("???".to_string(), "???".to_string()),
                None => ("???".to_string(), "???".to_string())
            };
            new_packets.push(Packet {
                timestamp,
                protocol: protocol.into(),
                source: format!("{}:{}", source_ip, source_port),
                destination: format!("{}:{}", destination_ip, destination_port),
                size: BytesFormat::format_bytes(packet.len() as f64)
            });

            if Instant::now() > deadline || new_packets.len() >= 1000 { break; };
        }

        new_packets
    } 
}
