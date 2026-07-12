use std::net::IpAddr;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use etherparse::{NetSlice, SlicedPacket, TransportSlice};
use pcap::{Active, Capture, Device};

use crate::config::Config;
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

                let Some(device) = Self::resolve_interface(Config::load().interface.as_deref(), &devices) else {
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

    fn resolve_interface(config_interface: Option<&str>, devices: &[Device]) -> Option<Device> {
        if let Some(interface_name) = config_interface {
            return devices.iter().find(|device| device.name == interface_name).cloned();
        }
        Self::auto_detect_device(devices)
    }

    fn auto_detect_device(devices: &[Device]) -> Option<Device> {
        devices
            .iter()
            .filter(|device| device.flags.is_up() && device.flags.is_running() && !device.flags.is_loopback() && !device.addresses.is_empty())
            .min_by_key(|device| {
                let has_routable_ipv4 = device
                    .addresses
                    .iter()
                    .any(|address| if let IpAddr::V4(ipv4) = address.addr { !ipv4.is_link_local() } else { false });
                let connected = matches!(device.flags.connection_status, pcap::ConnectionStatus::Connected);
                let interface_score = Self::interface_type_score(&device.name);
                (if has_routable_ipv4 { 0 } else { 2 }, if connected { 0 } else { 1 }, interface_score)
            })
            .cloned()
    }

    fn interface_type_score(name: &str) -> u8 {
        match () {
            _ if name.starts_with("en") => 0,
            _ if name.starts_with("utun") => 2,
            _ if name.starts_with("awdl") => 2,
            _ if name.starts_with("llw") => 2,
            _ if name.starts_with("pdp_ip") => 2,
            _ if name.starts_with("gif") => 2,
            _ if name.starts_with("stf") => 2,
            _ => 1,
        }
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
        let mut visited = Vec::new();
        Self::resolve_dns_labels(packet_payload, 12, &mut labels, &mut visited)?;

        if labels.is_empty() {
            return None;
        }
        Some(labels.join("."))
    }

    fn resolve_dns_labels<'payload>(payload: &'payload [u8], start: usize, labels: &mut Vec<&'payload str>, visited: &mut Vec<usize>) -> Option<()> {
        if visited.contains(&start) {
            return None;
        }

        if start >= payload.len() {
            return None;
        }

        visited.push(start);

        let mut offset = start;
        while offset < payload.len() {
            let size = payload[offset] as usize;
            if size == 0 {
                break;
            }

            let is_compressed_pointer = size & 0xC0 == 0xC0;
            if is_compressed_pointer {
                if offset + 1 >= payload.len() {
                    return None;
                }

                // Strip 0xC0 marker bits, merge with next byte to get the real offset
                let pointer = ((size & 0x3F) << 8) | payload[offset + 1] as usize;
                return Self::resolve_dns_labels(payload, pointer, labels, visited);
            }

            offset += 1;
            if offset + size > payload.len() {
                return None;
            }

            let label = std::str::from_utf8(&payload[offset..offset + size]).ok()?;
            labels.push(label);
            offset += size;
        }

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_dns_query_for_google_com_then_returns_domain() {
        let payload: &[u8] = &[
            0x12, 0x34, // Transaction ID
            0x01, 0x00, // Flags (standard query)
            0x00, 0x01, // QDCOUNT = 1
            0x00, 0x00, // ANCOUNT = 0
            0x00, 0x00, // NSCOUNT = 0
            0x00, 0x00, // ARCOUNT = 0
            0x06, b'g', b'o', b'o', b'g', b'l', b'e', // "google"
            0x03, b'c', b'o', b'm', // "com"
            0x00, // null terminator
            0x00, 0x01, // QTYPE = A
            0x00, 0x01, // QCLASS = IN
        ];
        let domain = PacketStream::parsed_dns_domain(payload);
        assert_eq!(domain, Some("google.com".to_string()));
    }

    #[test]
    fn given_short_payload_then_parsed_dns_domain_returns_none() {
        let payload: &[u8] = &[0x00; 10];
        assert_eq!(PacketStream::parsed_dns_domain(payload), None);
    }

    #[test]
    fn given_zero_qdcount_then_parsed_dns_domain_returns_none() {
        let payload: &[u8] = &[
            0x12, 0x34, // Transaction ID
            0x01, 0x00, // Flags
            0x00, 0x00, // QDCOUNT = 0
            0x00, 0x00, // ANCOUNT
            0x00, 0x00, // NSCOUNT
            0x00, 0x00, // ARCOUNT
        ];
        assert_eq!(PacketStream::parsed_dns_domain(payload), None);
    }

    #[test]
    fn given_dns_query_with_subdomain_then_returns_full_domain() {
        let payload: &[u8] = &[
            0x12, 0x34, // Transaction ID
            0x01, 0x00, // Flags
            0x00, 0x01, // QDCOUNT = 1
            0x00, 0x00, // ANCOUNT
            0x00, 0x00, // NSCOUNT
            0x00, 0x00, // ARCOUNT
            0x03, b'w', b'w', b'w', // "www"
            0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e', // "example"
            0x03, b'c', b'o', b'm', // "com"
            0x00, // null terminator
            0x00, 0x01, // QTYPE
            0x00, 0x01, // QCLASS
        ];
        let domain = PacketStream::parsed_dns_domain(payload);
        assert_eq!(domain, Some("www.example.com".to_string()));
    }

    #[test]
    fn given_compressed_label_pointer_then_resolves_full_domain() {
        let payload: &[u8] = &[
            0x12, 0x34, // Transaction ID
            0x01, 0x00, // Flags
            0x00, 0x01, // QDCOUNT = 1
            0x00, 0x00, // ANCOUNT
            0x00, 0x00, // NSCOUNT
            0x00, 0x00, // ARCOUNT
            0x03, b'w', b'w', b'w', // "www"   (offset 12-15)
            0xc0, 0x12, // pointer to offset 18 (0x12)
            0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e', // "example" (offset 18-25)
            0x03, b'c', b'o', b'm', // "com"    (offset 26-29)
            0x00, // null     (offset 30)
            0x00, 0x01, // QTYPE
            0x00, 0x01, // QCLASS
        ];
        let domain = PacketStream::parsed_dns_domain(payload);
        assert_eq!(domain, Some("www.example.com".to_string()));
    }

    #[test]
    fn given_pointer_cycle_then_returns_none() {
        let payload: &[u8] = &[
            0x12, 0x34, // Transaction ID
            0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, b'w', b'w', b'w', // "www"   (offset 12-15)
            0xc0, 0x0c, // pointer back to offset 12 (self)
        ];
        assert_eq!(PacketStream::parsed_dns_domain(payload), None);
    }

    #[test]
    fn given_pointer_past_end_then_returns_none() {
        let payload: &[u8] = &[
            0x12, 0x34, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, b'w', b'w', b'w', 0xc0, 0xFF, // pointer to offset 63 — past end of buffer
        ];
        assert_eq!(PacketStream::parsed_dns_domain(payload), None);
    }
}
