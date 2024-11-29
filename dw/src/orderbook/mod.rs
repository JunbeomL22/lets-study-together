use crate::PayloadParser::ParsedValue;
use crate::payload_field::PayloadField;
use pcap::Packet;
use crate::PayloadParser;

pub struct PacketAnalysisResult {
    pub non_zero_ratio: f64,
    pub average_23: f64,
    pub average_24: f64,
    pub mid_prices: Vec<f64>,
}

pub struct PacketAnalyzer;

impl PacketAnalyzer {
    pub fn analyze_packets(
        packets: &[Packet],
        fields: &[PayloadField],
        idx_23: usize,
        idx_24: usize,
    ) -> PacketAnalysisResult {
        let mut count_non_zero = 0;
        let mut sum_23 = 0.0;
        let mut sum_24 = 0.0;
        let mut mid_prices = Vec::new();

        for packet in packets {
            if let (Some(value_23), Some(value_24)) = (
                PayloadParser::parse_packet(packet, fields, idx_23),
                PayloadParser::parse_packet(packet, fields, idx_24),
            ) {
                // Extract numerical values if they are Double or Integer
                let value_23_num = match value_23 {
                    ParsedValue::Double(v) => v,
                    ParsedValue::Integer(v) => v as f64,
                    _ => continue,
                };

                let value_24_num = match value_24 {
                    ParsedValue::Double(v) => v,
                    ParsedValue::Integer(v) => v as f64,
                    _ => continue,
                };

                // Check if both values are non-zero
                if value_23_num != 0.0 || value_24_num != 0.0 {
                    count_non_zero += 1;
                    sum_23 += value_23_num;
                    sum_24 += value_24_num;

                    // Calculate mid price and store
                    let mid_price = (value_23_num + value_24_num) / 2.0;
                    mid_prices.push(mid_price);
                }
            }
        }

        let total_packets = packets.len() as f64;
        let non_zero_ratio = if total_packets > 0.0 {
            count_non_zero as f64 / total_packets
        } else {
            0.0
        };

        let average_23 = if count_non_zero > 0 {
            sum_23 / count_non_zero as f64
        } else {
            0.0
        };

        let average_24 = if count_non_zero > 0 {
            sum_24 / count_non_zero as f64
        } else {
            0.0
        };

        PacketAnalysisResult {
            non_zero_ratio,
            average_23,
            average_24,
            mid_prices,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payload_field::PayloadField;
    use pcap::Capture;
    use std::fs::File;
    use std::io::{self, Write};

    #[test]
    fn test_analyze_packets() -> Result<(), Box<dyn std::error::Error>> {
        let csv_path = "data/BF606F.csv";
        let pcap_path = "data/USD_Fwd_data.pcap";

        if !std::path::Path::new(csv_path).exists() || !std::path::Path::new(pcap_path).exists() {
            println!("Required file(s) not found.");
            return Ok(());
        }

        let fields = PayloadField::load_from_csv(csv_path)?;
        
        let mut capture = Capture::from_file(pcap_path)?;

        let mut packets = Vec::new();

        while let Ok(packet) = capture.next_packet() {
            packets.push(packet);
        }

        let idx_23 = 23;
        let idx_24 = 24;

        let analyze_packets = PacketAnalyzer::analyze_packets(&packets, &fields, idx_23, idx_24);
        let result = analyze_packets;

        // Print basic statistics
        println!("Non-zero Ratio: {:.2}%", result.non_zero_ratio * 100.0);
        println!("Average of 23: {:.2}", result.average_23);
        println!("Average of 24: {:.2}", result.average_24);

        // Print first 10 mid prices
        let display_count = 10;
        println!("Mid Prices (first {} entries):", display_count);
        for (i, price) in result.mid_prices.iter().take(display_count).enumerate() {
            println!("{:2}. {:.6}", i + 1, price);
        }

        // Save all mid prices to mid_price.txt
        save_mid_prices_to_file(&result.mid_prices, "mid_price.txt")?;

        Ok(())
    }

    fn save_mid_prices_to_file(mid_prices: &[f64], filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        for price in mid_prices {
            writeln!(file, "{:.6}", price)?;
        }
        Ok(())
    }
}