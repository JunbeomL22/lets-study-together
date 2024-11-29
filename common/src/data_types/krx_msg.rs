use mongodb::bson::{Binary, spec::BinarySubtype};
use std::{fmt, str};
use serde::{Deserialize, Serialize};
use encoding_rs::EUC_KR;
use crate::UnixNano;
use crate::data_type::{
    krx_messages_instcode_range,
    krx_message_dist_index_range,
};

/// # Arguments
/// * `date` - yyyymmdd
/// * `trcode` - 5 bytes (first two bytes are data type, last three bytes are asset code, e.g., B606F)
/// * `instcode` - 12 bytes (e.g., KR4165N30007)
/// * `dist_index` - distribution index, the order of the message regarding the same trcode.
/// * `packet_timestamp` - UnixNano (the time when the packet is received)
/// * `timestamp` - UnixNano (the time when the message is received on the processor)
/// * `payload` - binary data
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
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            self.date / 10000,
            (self.date % 10000) / 100,
            self.date % 100
        );

        let payload_len = self.payload.len();

        // 1. EUC-KR/CP949로 디코딩 시도
        let (cow, _, _) = EUC_KR.decode(&self.payload[..payload_len.saturating_sub(1)]);
        let payload_str = cow.into_owned();

        write!(
            f,
            "KrxMsg {{\n  date: {}\n  trcode: {}\n  distidx: {}\n instcode: {}\n  packet_timestamp: {}\n  timestamp: {}\n  payload: {} ({} bytes)\n}}",
            date_str,
            self.trcode,
            self.distidx.map_or("None".to_string(), |idx| idx.to_string()),
            self.instcode.as_deref().unwrap_or("None"),
            self.packet_timestamp.map_or("None".to_string(), |ts| ts.to_string()),
            self.timestamp.map_or("None".to_string(), |ts| ts.to_string()),
            payload_str,
            payload_len,
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
