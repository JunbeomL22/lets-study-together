// Jay 
// unused import -> run 'cargo check' and 'cargo clippy' before push
//use std::error::Error;
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
    // Jay
    // Box<dyn Error> is not thread safe.
    // Moreover, Box<dyn Trait> is slow. General practice is specify the error type in lib and use anyhow in the application
    pub fn load_from_csv(file_path: &str) -> Result<Vec<PayloadField>, std::io::Error> {
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
            // Jay
            // run 'cargo clippy' before push
            let length = record.get(4)
                .map(|s| if s.trim().is_empty() { "0" } else { s.trim() })
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0);

            let cumulative_length = record.get(5)
                .map(|s| if s.trim().is_empty() { "0" } else { s.trim() })
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0);

            let start_point = record.get(6)
                .map(|s| if s.trim().is_empty() { "0" } else { s.trim() })
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
   //use anyhow::Context;

   #[test]
   fn test_load_from_csv() -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        println!("Current dir: {:?}", current_dir);
        let fields = PayloadField::load_from_csv("../data/BF606F_new.csv")
            .map_err(|e| anyhow::anyhow!("Failed to load CSV file: {}", e))?;
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

        Ok(())
   }
}