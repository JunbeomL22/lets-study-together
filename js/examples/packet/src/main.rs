use pnet::packet::ethernet::EtherType;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pcap::Capture;
use serde_json::json;
use std::fs::File;
use std::io::Write;

const LOCAL_LOOPBACK: EtherType = EtherType(32785);

#[derive(Debug, Clone)]
pub struct PacketExtractor {
    file_input: String,
    file_output: String,
    header_filter: Option<Vec<String>>,
}

impl PacketExtractor {
    pub fn new(file_input: String, file_output: String, header_filter: Option<Vec<String>>) -> PacketExtractor {
        PacketExtractor {
            file_input,
            file_output,
            header_filter,
        }
    }

    pub fn extract_to_json(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cap = Capture::from_file(&self.file_input)?;
        let mut packets = Vec::new();
        
        // Process each packet
        while let Ok(packet) = cap.next_packet() {
            if let Some(payload) = self.extract_payload(packet.data) {
                // Convert payload bytes to ASCII string
                let ascii_payload: String = payload
                    .iter()
                    .map(|&b| b as char)
                    .collect();
                
                let packet_json = json!({
                    "timestamp": packet.header.ts.tv_sec,
                    "microseconds": packet.header.ts.tv_usec,
                    "payload": ascii_payload,
                });
                
                packets.push(packet_json);
            }
        }
        
        // Create final JSON
        let json_output = json!({
            "packets": packets
        });
        
        // Write to JSON file
        let mut file = File::create(&self.file_output)?;
        file.write_all(serde_json::to_string_pretty(&json_output)?.as_bytes())?;
        
        Ok(())
    }

    fn extract_payload(&self, packet: &[u8]) -> Option<Vec<u8>> {
        if let Some(ethernet_packet) = EthernetPacket::new(packet) {
            if ethernet_packet.get_ethertype() == pnet::packet::ethernet::EtherTypes::Ipv4 {
                if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                    match ipv4_packet.get_next_level_protocol() {
                        IpNextHeaderProtocols::Udp => {
                            if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                                let payload = udp_packet.payload().to_vec();
                                return self.filter_payload(payload);
                            }
                        },
                        IpNextHeaderProtocols::Tcp => {
                            if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                                let payload = tcp_packet.payload().to_vec();
                                return self.filter_payload(payload);
                            }
                        }
                        _ => return None,
                    }
                }
            } else if ethernet_packet.get_ethertype() == LOCAL_LOOPBACK && ethernet_packet.payload().len() >= 18 {
                let payload = ethernet_packet.payload()[18..].to_vec();
                return self.filter_payload(payload);
            }
        }
        None
    }

    fn filter_payload(&self, payload: Vec<u8>) -> Option<Vec<u8>> {
        match &self.header_filter {
            Some(filter) => {
                for header in filter {
                    if payload.starts_with(header.as_bytes()) {
                        return Some(payload);
                    }
                }
                None
            }
            None => Some(payload),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let packet_extractor = PacketExtractor::new(
        "D:\\HFT\\Koscom Data\\koscom_udp_2024-09-27_filtered.pcap".to_string(),
        "D:\\HFT\\Koscom Data\\practice\\output.json".to_string(),
        Some(vec!["B6".to_string(), "G7".to_string(), "A3".to_string()])
    );
    packet_extractor.extract_to_json()?;
    Ok(())
}