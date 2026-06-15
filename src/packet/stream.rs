use super::resource::Packet;

pub struct PacketStream;

impl PacketStream {
    pub fn get_packets() -> Vec<Packet> {
       Self::mock_data() 
    }

    fn mock_data() -> Vec<Packet> { 
        vec![
            Packet {
                timestamp: "14:32:01".into(),
                protocol: "TCP".into(),
                source: "192.168.1.100:54321".into(),
                destination: "142.250.80.46:443".into(),
                size: "1.2 KB".into(),
            },
            Packet {
                timestamp: "14:32:02".into(),
                protocol: "UDP".into(),
                source: "192.168.1.100:5353".into(),
                destination: "224.0.0.251:5353".into(),
                size: "120 B".into(),
            },
            Packet {
                timestamp: "14:32:03".into(),
                protocol: "DNS".into(),
                source: "192.168.1.100:61023".into(),
                destination: "8.8.8.8:53".into(),
                size: "280 B".into(),
            },
        ]
    }
}
