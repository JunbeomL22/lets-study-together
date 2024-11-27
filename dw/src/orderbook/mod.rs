use pcap::Capture;
use std::error::Error;
use crate::payload_field::PayloadField;

#[derive(Debug)]
pub struct OrderBook {
    stock_code: String,
    ask_price1: f64,
    ask_volume1: i32,
    ask_order_count1: i32,
    bid_price1: f64,
    bid_volume1: i32,
    bid_order_count1: i32,
    // 필요한 추가 필드들...
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            stock_code: String::new(),
            ask_price1: 0.0,
            ask_volume1: 0,
            ask_order_count1: 0,
            bid_price1: 0.0,
            bid_volume1: 0,
            bid_order_count1: 0,
        }
    }

    pub fn process_pcap_file(input_file: &str) -> Result<Vec<OrderBook>, Box<dyn Error>> {
     

        let mut capture = Capture::from_file(input_file)?;
        let mut order_books = Vec::new();
        
        // PayloadField 정보 로드
        let fields = PayloadField::load_from_csv("data/BF606F.csv")?;
        
        let mut count = 0;

        while let Ok(packet) = capture.next_packet() {
            if packet.data.len() > 42 {

                let payload = &packet.data[42..];
                let search_position = &payload[18..];
                let order_book = Self::parse_order_book(payload, &fields)?;

                order_books.push(order_book);
                
                count += 1;
                if count >= 100 {
                    break;
                }
            }
        }

        Ok(order_books)
    }


    fn parse_order_book(payload: &[u8], fields: &[PayloadField]) -> Result<OrderBook, Box<dyn Error>> {
        // fields 정보를 이용하여 payload의 각 필드 파싱
        let mut order_book = OrderBook::new();
        
        for field in fields.iter().skip(22).take(4) {
            let start: usize = field.start_point as usize;
            let end: usize = start + field.length as usize;
            
            if end <= payload.len() {
                let data = &payload[start..end];
                
                match field.data_type.to_lowercase().as_str() {
                    "double" => {
                        let value = String::from_utf8_lossy(data)
                            .trim()
                            .parse::<f64>()
                            .unwrap_or(0.0);
                        println!("Parsed double: {}", value); // 값 출력

                        // value 사용

                    },
                    "int" => {
                        let value = String::from_utf8_lossy(data)
                            .trim()
                            .parse::<i32>()
                            .unwrap_or(0);
                        // value 사용
                        println!("Parsed int: {}", value); // 값 출력
                    },
                    "string" => {
                        let value = String::from_utf8_lossy(data)
                            .trim()
                            .to_string();
                        // value 사용
                        println!("Parsed Str: {}", value); // 값 출력

                    },
                    _ =>  {
                        println!("Unknown data type: {}", field.data_type);
                    }
                }
            }
        }

        Ok(order_book)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_pcap_file() {
        let result = OrderBook::process_pcap_file("data/output.pcap");
        assert!(result.is_ok());
        
        let order_books = result.unwrap();
        assert!(!order_books.is_empty());
        
        // 첫 번째 orderbook 데이터 출력
        println!("First OrderBook: {:#?}", &order_books[0]);
    }
}