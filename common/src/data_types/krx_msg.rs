use mongodb::bson::{Binary, spec::BinarySubtype};
use std::{fmt, str};
use serde::{Deserialize, Deserializer, Serialize};
use encoding_rs::EUC_KR;
use crate::UnixNano;
use crate::data_types::{
    krx_messages_instcode_range,
    krx_message_dist_index_range,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KrxMsg {
    pub date: i32,
    pub trcode: String,
    pub distidx: Option<i32>,
    pub instcode: Option<String>,
    pub packet_timestamp: Option<UnixNano>,
    pub timestamp: Option<UnixNano>,
    #[serde(with = "binary_serde")]
    pub payload: Vec<u8>,
}

impl fmt::Display for KrxMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KrxMsg {{ date: {}, trcode: {}, distidx: {}, instcode: {}, packet_timestamp: {}, timestamp: {}, payload: {} bytes }}",
            self.date,
            self.trcode,
            self.distidx.map_or("None".to_string(), |idx| idx.to_string()),
            self.instcode.as_deref().unwrap_or("None"),
            self.packet_timestamp.map_or("None".to_string(), |ts| ts.to_string()),
            self.timestamp.map_or("None".to_string(), |ts| ts.to_string()),
            self.payload.len()
        )
    }
}



impl KrxMsg {
    pub fn new_from_payload(
        date: i32, 
        payload: &[u8], 
        packet_timestamp: Option<UnixNano>,
        timestamp: Option<UnixNano>,
    ) -> Result<Self, std::string::FromUtf8Error> {
        let trcode = if let Ok(trcode) = String::from_utf8(payload[..5].to_vec()) {
            trcode
        } else {
            let pay_clone = payload.to_vec();
            flashlog::flash_info!("DECODE";"Failed to decode trcode"; payload = pay_clone);
            "".to_string()
        };
        let instcode_range = krx_messages_instcode_range(payload);
        let instcode = match instcode_range {
            Some(range) => {
                if payload.len() < range.end {
                    None
                } else {
                    match String::from_utf8(payload[range.start..range.end].to_vec()) {
                        Ok(instcode) => Some(instcode),
                        Err(_) => {
                            let pay_clone = payload.to_vec();
                            flashlog::flash_info!("DECODE";"Failed to decode instcode"; payload = pay_clone);
                            None
                        }
                    }
                }
            },
            None => None,
        };

        let distidx_range = krx_message_dist_index_range(payload);
        let distidx: Option<i32> = match distidx_range {
            Some(range) => {
                if payload.len() < range.end {
                    None
                } else {
                    let clipped = &payload[range.start..range.end];
                    let res = match String::from_utf8_lossy(clipped).parse::<i32>() {
                        Ok(distidx) => Some(distidx),
                        Err(_) => {
                            None
                            
                        }
                    };
                    res
                }
            },
            None => None,
        };

        Ok(Self {
            date,
            trcode,
            distidx,
            instcode,
            packet_timestamp,
            timestamp,
            payload: payload.to_vec(),
        })
    }
}


// KrxMsgStream 구현
struct KrxMsgStream<F>(F);

impl<'de, F: FnMut(KrxMsg)> serde::de::DeserializeSeed<'de> for KrxMsgStream<F> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(KrxMsgVisitor(self.0))
    }
}

// Visitor 구현
struct KrxMsgVisitor<F>(F);

impl<'de, F: FnMut(KrxMsg)> serde::de::Visitor<'de> for KrxMsgVisitor<F> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of KrxMsg")
    }

    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        while let Some(msg) = seq.next_element()? {
            (self.0)(msg);
        }
        Ok(())
    }
}

mod binary_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Binary {
            subtype: BinarySubtype::Generic,
            bytes: bytes.to_vec(),
        }.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Binary::deserialize(deserializer).map(|binary| binary.bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
<<<<<<< HEAD
    use std::io::BufReader;
    use serde::de::DeserializeSeed;

    #[test]
    fn test_stream_krx_msgs() {
        let file_name = "../data/multiasset_db.krx_msg.json";
        let file = std::fs::File::open(file_name).expect("파일을 열 수 없습니다");
        let reader = BufReader::new(file);
        
        // 처음 두 메시지만 처리하기 위한 카운터
        let mut count = 0;
        let mut first_two_msgs = Vec::new();

        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        KrxMsgStream(|msg: KrxMsg| {
            if count < 2 {
                println!("메시지 #{}", count + 1);
                println!("{}", msg);
                first_two_msgs.push(msg);
                count += 1;
            }
        })
        .deserialize(&mut deserializer)
        .expect("스트리밍 처리 실패");
=======
    use struson::reader::{JsonStreamReader, JsonReader};

    #[test]
    fn test_deserialized_krx_msg() -> anyhow::Result<()> {
        // read from multiasset_db.krx_msg.json
        let file_name = "../data/multiasset_db.krx_msg.json";
        let file_path = format!("{}", file_name);
        let file = std::fs::File::open(file_path).unwrap();
        let reader = std::io::BufReader::new(file);
        /* non-streaming 
        let krx_msgs: Vec<KrxMsg> = serde_json::from_reader(reader).unwrap();
        for krx_msg in krx_msgs {
            println!("{}", krx_msg);
        }
        */
        // streaming
        let mut stream_reader = JsonStreamReader::new(reader);
        stream_reader.begin_array()?;
        
        while stream_reader.has_next()? {
            let krx_msg: KrxMsg = stream_reader.deserialize_next()?;
            println!("{}", krx_msg);
        }

        stream_reader.end_array()?;
        
        Ok(())
>>>>>>> 44a4ea3a315723a24529263daf612a63ec3a5535

        // 검증
        assert_eq!(first_two_msgs.len(), 2, "첫 두 개의 메시지를 읽어야 합니다");
        
        // 첫 번째 메시지 검증
        assert_eq!(first_two_msgs[0].trcode, "B606F");
        
        // 메모리 사용량 출력 (선택적)
        let memory_size = std::mem::size_of_val(&first_two_msgs);
        println!("처리된 데이터의 메모리 크기: {} bytes", memory_size);
    }

    #[test]
    fn test_quick_file_check() {
        use std::io::Read;
        
        let file_name = "../data/multiasset_db.krx_msg.json";
        let metadata = std::fs::metadata(file_name).expect("파일 정보를 읽을 수 없습니다");
        println!("파일 크기: {} bytes", metadata.len());
        
        // 파일의 처음 부분만 읽어서 확인
        let mut file = std::fs::File::open(file_name).expect("파일을 열 수 없습니다");
        let mut buffer = vec![0; 1024];
        let n = file.read(&mut buffer).expect("읽기 실패");
        let start = String::from_utf8_lossy(&buffer[..n]);
        
        println!("파일 시작 부분 (처음 200자):\n{}", &start[..200.min(start.len())]);
        assert!(start.starts_with("[{"), "JSON 배열 형식이어야 합니다");
    }
}