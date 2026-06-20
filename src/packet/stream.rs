use std::net::IpAddr;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use etherparse::{NetSlice, SlicedPacket, TransportSlice};
use pcap::{Active, Capture, Device};

use crate::utilities::bytes_format::BytesFormat;

use super::resource::Packet;

const DNS_PORT: u16 = 53;

static CAPTURE: Mutex<Option<Capture<Active>>> = Mutex::new(None);
static CAPTURE_START: Mutex<Option<Instant>> = Mutex::new(None);

/*
 * Captures live network packets using pcap. On first call, discovers
 * a non-loopback interface with a routable IPv4 address and opens a
 * promiscuous capture. Each subsequent call collects packets for up
 * to 100ms or 1000 packets, parsed with etherparse into transport
 * (TCP/UDP/ICMP) and network (IPv4/IPv6/ARP) layers. DNS domain names
 * are resolved from UDP port 53 payloads.
 */
pub struct PacketStream;

impl PacketStream {
    pub fn get_packets() -> Vec<Packet> {
        let mut new_packets: Vec<Packet> = Vec::new();
        let mut capture_guard = CAPTURE.lock().unwrap();

        let capture = match capture_guard.as_mut() {
            Some(c) => c,
            None => {
                let Some(devices) = Device::list().ok() else {
                    return Vec::new();
                };
                let Some(device) = devices
                    .iter()
                    .filter(|device| device.flags.is_up() && device.flags.is_running() && !device.flags.is_loopback() && !device.addresses.is_empty())
                    .min_by_key(|device| {
                        let has_routable_ipv4 = device
                            .addresses
                            .iter()
                            .any(|address| if let IpAddr::V4(ipv4) = address.addr { !ipv4.is_link_local() } else { false });
                        if has_routable_ipv4 { 0 } else { 1 }
                    })
                else {
                    return Vec::new();
                };

                let Ok(inactive) = Capture::from_device(device.clone()) else {
                    return Vec::new();
                };
                let Some(active) = inactive.promisc(true).snaplen(65535).timeout(100).open().ok() else {
                    return Vec::new();
                };
                *CAPTURE_START.lock().unwrap() = Some(Instant::now());
                capture_guard.insert(active)
            }
        };

        let deadline = Instant::now() + Duration::from_millis(100);

        while let Ok(packet) = capture.next_packet() {
            let Ok(parsed_packet) = SlicedPacket::from_ethernet(packet.data) else {
                continue;
            };
            let start = *CAPTURE_START.lock().unwrap().get_or_insert(Instant::now());
            let timestamp = format!("+{:.3}s", start.elapsed().as_secs_f64());

            let (protocol, source_port, destination_port, dns_domain) = match parsed_packet.transport {
                Some(TransportSlice::Tcp(tcp)) => ("TCP", tcp.source_port(), tcp.destination_port(), None),
                Some(TransportSlice::Udp(udp)) => {
                    let source_port = udp.source_port();
                    let destination_port = udp.destination_port();
                    let dns_domain = if source_port == DNS_PORT || destination_port == DNS_PORT {
                        Self::parsed_dns_domain(udp.payload())
                    } else {
                        None
                    };
                    ("UDP", source_port, destination_port, dns_domain)
                }
                Some(TransportSlice::Icmpv4(_)) => ("ICMPV4", 0, 0, None),
                Some(TransportSlice::Icmpv6(_)) => ("ICMPV6", 0, 0, None),
                None => ("???", 0, 0, None),
            };
            let (source_ip, destination_ip) = match parsed_packet.net {
                Some(NetSlice::Ipv4(ipv4)) => (ipv4.header().source_addr().to_string(), ipv4.header().destination_addr().to_string()),
                Some(NetSlice::Ipv6(ipv6)) => (ipv6.header().source_addr().to_string(), ipv6.header().destination_addr().to_string()),
                Some(NetSlice::Arp(_)) => ("???".to_string(), "???".to_string()),
                None => ("???".to_string(), "???".to_string()),
            };
            new_packets.push(Packet {
                timestamp,
                protocol: protocol.into(),
                source: format!("{}:{}", source_ip, source_port),
                destination: format!("{}:{}", destination_ip, destination_port),
                size: BytesFormat::format_bytes(packet.len() as f64),
                source_port,
                destination_port,
                raw_size: packet.len() as u64,
                dns_domain,
            });

            if Instant::now() > deadline || new_packets.len() >= 1000 {
                break;
            };
        }

        new_packets
    }

    fn parsed_dns_domain(packet_payload: &[u8]) -> Option<String> {
        if packet_payload.len() < 12 {
            return None;
        }
        let qdcount = u16::from_be_bytes([packet_payload[4], packet_payload[5]]) as usize;
        if qdcount == 0 {
            return None;
        }

        let mut labels: Vec<&str> = Vec::new();
        let mut offset = 12;

        while offset < packet_payload.len() {
            let size = packet_payload[offset] as usize;
            if size == 0 {
                break;
            }

            if size & 0xC0 == 0xC0 {
                break;
            }

            offset += 1;
            if offset + size > packet_payload.len() {
                return None;
            }

            let label = std::str::from_utf8(&packet_payload[offset..offset + size]).ok()?;
            labels.push(label);
            offset += size;
        }

        if labels.is_empty() {
            return None;
        }
        Some(labels.join("."))
    }
}
