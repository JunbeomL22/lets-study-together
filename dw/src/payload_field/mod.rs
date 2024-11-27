use std::error::Error;
use std::fs::File;
use std::io::Read;
use encoding_rs::EUC_KR;
use csv::ReaderBuilder;

#[derive(Debug)]
pub struct PayloadField {
   pub korean_name: String,
   pub item_name: String,
   pub sub_section: String,
   pub data_type: String,
   pub length: i32,
   pub cumulative_length: i32,
   pub start_point: i32,
}

impl PayloadField {
   pub fn load_from_csv(file_path: &str) -> Result<Vec<PayloadField>, Box<dyn Error>> {
       // 파일을 바이트로 읽기
       let mut file = File::open(file_path)?;
       let mut bytes = Vec::new();
       file.read_to_end(&mut bytes)?;

       // EUC-KR에서 UTF-8로 변환
       let (cow, _, _) = EUC_KR.decode(&bytes);

       // CSV 파서 설정
       let mut rdr = ReaderBuilder::new()
           .has_headers(true)  // 헤더 처리 추가
           .from_reader(cow.as_bytes());

       let mut payload_fields = Vec::new();

       for result in rdr.records() {
           let record = result?;
           
           // 빈 문자열은 "0"으로 처리하고 공백 제거
           let length = record.get(4)
               .and_then(|s| if s.trim().is_empty() { Some("0") } else { Some(s.trim()) })
               .unwrap_or("0")
               .parse::<i32>()
               .unwrap_or(0);

           let cumulative_length = record.get(5)
               .and_then(|s| if s.trim().is_empty() { Some("0") } else { Some(s.trim()) })
               .unwrap_or("0")
               .parse::<i32>()
               .unwrap_or(0);

           let start_point = record.get(6)
               .and_then(|s| if s.trim().is_empty() { Some("0") } else { Some(s.trim()) })
               .unwrap_or("0")
               .parse::<i32>()
               .unwrap_or(0);

           let field = PayloadField {
               korean_name: record.get(0).unwrap_or("").to_string(),
               item_name: record.get(1).unwrap_or("").to_string(),
               sub_section: record.get(2).unwrap_or("").to_string(),
               data_type: record.get(3).unwrap_or("").to_string(),
               length,
               cumulative_length,
               start_point,
           };

           payload_fields.push(field);
       }

       Ok(payload_fields)
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_load_from_csv() {
       let fields = PayloadField::load_from_csv("data/BF606F.csv").unwrap();
       assert!(!fields.is_empty());
       
       // 디버깅을 위한 출력 추가
       println!("First field contents:");
       println!("Korean name: {}", fields[0].korean_name);
       println!("Item name: {}", fields[0].item_name);
       println!("Length: {}", fields[0].length);
       println!("Cumulative length: {}", fields[0].cumulative_length);
       println!("Start point: {}", fields[0].start_point);
       
       let first = &fields[0];
       assert_eq!(first.length, 2, "Expected length to be 2, got {}", first.length);
       assert_eq!(first.cumulative_length, 2);
       assert_eq!(first.start_point, 0);
   }
}