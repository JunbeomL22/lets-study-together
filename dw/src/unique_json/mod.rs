use crate::payload_field::PayloadField;
use crate::payload_parser;
use common::KrxMsg;



#[cfg(test)]
mod tests {
    use super::*;
    use struson::reader::{JsonStreamReader, JsonReader};
    use common::KrxMsg;
    use pcap::Capture;

    #[test]

    fn stream_json_with_parser() -> anyhow::Result<()> {
        // file 열기
        let file_name = "data/multiasset_db.krx_msg.json";
        let file = std::fs::File::open(file_name)?;
        let reader = std::io::BufReader::new(file);
        let mut stream_reader = JsonStreamReader::new(reader);

        let file_name_2 = "data/multiasset_db.krx_msg_2.json";
        let file_2 = std::fs::File::open(file_name_2)?;
        let reader_2 = std::io::BufReader::new(file_2);
        let mut stream_reader_2 = JsonStreamReader::new(reader_2);

        
        // csv 파일 존재 여부 체크
        let csv_path = "data/BF606F_new.csv";

        if !std::path::Path::new(csv_path).exists() {
            println!("CSV file not found: {}", csv_path);
            return Ok(());
        }


        // csv 파일 로드
        // Jay: unused 
        /* 
        let fields = PayloadField::load_from_csv(csv_path)
            .map_err(|e| anyhow::anyhow!("Failed to load CSV: {}", e.to_string()))?;
        */
        stream_reader.begin_array()?;
        stream_reader_2.begin_array()?;

        let mut cnt = 0;

        // Jay: This still itereates over all the elements. It passes only deserialization. 
        // Itration itself is also a very heavy task.
        // 처음 10개만 비교하고 나머지는 건너뛰기
        while stream_reader.has_next()? && stream_reader_2.has_next()? {
            let krx_msg: KrxMsg = stream_reader.deserialize_next()?;
            let krx_msg_2: KrxMsg = stream_reader_2.deserialize_next()?;
            println!("{}", krx_msg);
            println!("{}", krx_msg_2);

            assert_eq!(
                krx_msg.instcode,
                krx_msg_2.instcode,
                "instcode mismatch: {:?} != {:?}",
                krx_msg.instcode,
                krx_msg_2.instcode
            );
            
            /*
            else {
                // 10개 이후의 요소들은 건너뛰기
                stream_reader.skip_value()?;
                stream_reader_2.skip_value()?;
            }
            cnt += 1;
            */
            cnt += 1;
            if cnt >= 10 {
                break;
            }
        }

        // Jay: This is not needed if your intention is only iterate the first 10 elements.
        // stream_reader.end_array()?;
        // stream_reader_2.end_array()?;

        Ok(())

    }
}