use pnet::packet::ethernet::EtherType;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pcap::Capture;

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

    pub fn filter_packets_with_header(&self) {
        // Open PCAP file
        let mut cap = match Capture::from_file(&self.file_input) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Cannot open input file: {}", e);
                return;
            }
        };
    
        let output_cap = Capture::dead(cap.get_datalink()).unwrap();
        let mut savefile = output_cap.savefile(self.file_output.as_str()).unwrap();
    
        // Process each packet
        while let Ok(packet) = cap.next_packet() {
            if self.is_valid_packet(packet.data) {
                savefile.write(&packet);
            }
        }
    }

    fn is_valid_packet(&self, packet: &[u8]) -> bool {
        if let Some(ethernet_packet) = EthernetPacket::new(packet) {
            if ethernet_packet.get_ethertype() == pnet::packet::ethernet::EtherTypes::Ipv4 {
                if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                    match ipv4_packet.get_next_level_protocol() {
                        IpNextHeaderProtocols::Udp => {
                            if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                                let payload = udp_packet.payload();
                                match self.header_filter {
                                    Some(ref filter) => {
                                        for header in filter.iter() {
                                            if payload.starts_with(header.as_bytes()) {
                                                return true;
                                            }
                                        }
                                        return false;
                                    }
                                    None => return true,
                                }
                            }
                        },
                        IpNextHeaderProtocols::Tcp => {
                            if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                                let payload = tcp_packet.payload();
                                match self.header_filter {
                                    Some(ref filter) => {
                                        for header in filter.iter() {
                                            if payload.starts_with(header.as_bytes()) {
                                                return true;
                                            }
                                        }
                                        return false;
                                    }
                                    None => return true,
                                }
                            }
                        }
                        _ => {
                            return false;
                        }
                    }
                }
            } else if ethernet_packet.get_ethertype() == LOCAL_LOOPBACK && ethernet_packet.payload().len() >= 18 {
                let clipped = ethernet_packet.payload()[18..].to_vec();
                match self.header_filter {
                    Some(ref filter) => {
                        for header in filter.iter() {
                            if clipped.starts_with(header.as_bytes()) {
                                return true;
                            }
                        }
                        return false;
                    }
                    None => return true,
                }
            }
        }
        false
    }
}