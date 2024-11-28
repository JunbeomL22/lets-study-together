use pcap::Capture;
use crate::payload_field::PayloadField;

#[derive(Debug)]
pub enum ParsedValue {
   Double(f64),
   Integer(i32),
   Text(String),
}

pub struct PacketParser {
   fields: Vec<PayloadField>,
   skip_count: usize,
   max_count: usize,
}

impl PacketParser {
   pub fn new(csv_path: &str, skip_count: usize, max_count: usize) -> Result<Self, Box<dyn std::error::Error>> {
       let fields = PayloadField::load_from_csv(csv_path)?;
       Ok(PacketParser {
           fields,
           skip_count,
           max_count,
       })
   }

   pub fn parse_pcap(&self, pcap_path: &str, field_idx: usize) -> Result<Vec<ParsedValue>, Box<dyn std::error::Error>> {
       let mut capture = Capture::from_file(pcap_path)?;
       let mut parsed_values = Vec::new();
       let mut processed_count = 0;
       let mut skip_num = 0;

       while let Ok(packet) = capture.next_packet() {
           if skip_num < self.skip_count {
               skip_num += 1;
               continue;
           }

           if packet.data.len() > 42 {
               if let Some(parsed_value) = self.parse_packet(&packet, field_idx) {
                   parsed_values.push(parsed_value);
                   processed_count += 1;

                   if processed_count >= self.max_count {
                       break;
                   }
               }
           }
       }

       Ok(parsed_values)
   }

   fn parse_packet(&self, packet: &pcap::Packet, field_idx: usize) -> Option<ParsedValue> {
       let field = &self.fields[field_idx];
       let payload = &packet.data[42..];
       
       if payload.len() < field.cumulative_length as usize {
           return None;
       }

       let data = &payload[field.start_point as usize..field.cumulative_length as usize];
       Some(self.parse_data(data, &field.data_type))
   }

   fn parse_data(&self, data: &[u8], data_type: &str) -> ParsedValue {
       match data_type {
           "Double" => ParsedValue::Double(self.bytes_to_f64(data)),
           "Int" => ParsedValue::Integer(self.bytes_to_i32(data)),
           _ => ParsedValue::Text(self.bytes_to_string(data)),
       }
   }

   fn bytes_to_string(&self, bytes: &[u8]) -> String {
       bytes.iter()
           .map(|&b| b as char)
           .collect::<String>()
   }

   fn bytes_to_f64(&self, bytes: &[u8]) -> f64 {
       let mut result = 0.0;
       let mut is_negative = false;
       let mut i = 0;
       
       if bytes[0] == b'-' {
           is_negative = true;
           i = 1;
       }

       while i < bytes.len() && bytes[i].is_ascii_digit() {
           result = result * 10.0 + (bytes[i] - b'0') as f64;
           i += 1;
       }

       if i < bytes.len() && bytes[i] == b'.' {
           i += 1;
           let mut decimal = 0.1;
           while i < bytes.len() && bytes[i].is_ascii_digit() {
               result += (bytes[i] - b'0') as f64 * decimal;
               decimal *= 0.1;
               i += 1;
           }
       }

       if is_negative { -result } else { result }
   }

   fn bytes_to_i32(&self, bytes: &[u8]) -> i32 {
       let mut result = 0;
       let mut is_negative = false;
       let mut i = 0;

       if bytes[0] == b'-' {
           is_negative = true;
           i = 1;
       }

       while i < bytes.len() && bytes[i].is_ascii_digit() {
           result = result * 10 + (bytes[i] - b'0') as i32;
           i += 1;
       }

       if is_negative { -result } else { result }
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_packet_parser() -> Result<(), Box<dyn std::error::Error>> {
       let parser = PacketParser::new("data/BF606F.csv", 500, 20)?;
       let results = parser.parse_pcap("data/USD_Fwd_data.pcap", 8)?;
       
       assert!(!results.is_empty());
       println!("Parsed values: {:#?}", results);
       Ok(())
   }
}