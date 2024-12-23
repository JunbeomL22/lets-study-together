use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KrBenchmarkBond {
    /// 변경일자 (e.g., 20241210)
    #[serde(rename = "date")]
    pub date: i32,

    /// 변경내용 (조성해제, 조성지정, 지표해제, 지표지정)
    #[serde(rename = "change_type")]
    pub change_type: String,

    /// 만기 (년)
    #[serde(rename = "maturity_years")]
    pub maturity_years: i32,

    /// 종목명
    #[serde(rename = "bond_name")]
    pub bond_name: String,

    /// 표준코드
    #[serde(rename = "isin")]
    pub isin: String,

    /// 발행일 (e.g., 20231210)
    #[serde(rename = "issue_date")]
    pub issue_date: i32,

    /// 만기일 (e.g., 20331210)
    #[serde(rename = "maturity_date")]
    pub maturity_date: i32,

    /// 상장금액 (백만원)
    #[serde(rename = "issue_amount")]
    pub issue_amount: f64,

    /// 표면금리 (%)
    #[serde(rename = "coupon_rate")]
    pub coupon_rate: f64,
}

// Example JSON conversion function
impl KrBenchmarkBond {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use approx::assert_relative_eq;

    #[test]
    fn test_to_json() {
        let bond = KrBenchmarkBond {
            date: 20241210,
            change_type: "조성해제".to_string(),
            maturity_years: 10,
            bond_name: "국고채 3년".to_string(),
            isin: "KR4165N30007".to_string(),
            issue_date: 20231210,
            maturity_date: 20331210,
            issue_amount: 1000000.0,
            coupon_rate: 1.5,
        };
        let json = bond.to_json().unwrap();
        let expected = r#"{
  "date": 20241210,
  "change_type": "조성해제",
  "maturity_years": 10,
  "bond_name": "국고채 3년",
  "isin": "KR4165N30007",
  "issue_date": 20231210,
  "maturity_date": 20331210,
  "issue_amount": 1000000.0,
  "coupon_rate": 1.5
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_read_json_file() {
        let json = fs::read_to_string("data/bench.json").unwrap();
        let benchs: Vec<KrBenchmarkBond> = serde_json::from_str(&json).unwrap();

        let last_bench = benchs.last().unwrap();

        let date = last_bench.date;
        let change_type = &last_bench.change_type;
        let maturity_years = last_bench.maturity_years;
        let bond_name = &last_bench.bond_name;
        let isin = &last_bench.isin;
        let issue_date = last_bench.issue_date;
        let maturity_date = last_bench.maturity_date;
        let issue_amount = last_bench.issue_amount;
        let coupon_rate = last_bench.coupon_rate;

        // KrBenchmarkBond { date: 20240610, change_type: "지표지정", maturity_years: 3, bond_name: "국고03250-2706(24-4)", isin: "KR103501GE64", issue_date: 20240610, maturity_date: 20270610, issue_amount: 153580.0, coupon_rate: 3.25 }
        assert_eq!(date, 20240610);
        assert_eq!(change_type, "지표지정");
        assert_eq!(maturity_years, 3);
        assert_eq!(bond_name, "국고03250-2706(24-4)");
        assert_eq!(isin, "KR103501GE64");
        assert_eq!(issue_date, 20240610);
        assert_eq!(maturity_date, 20270610);
        assert_relative_eq!(issue_amount, 153580.0, epsilon=1.0e-6);
        assert_relative_eq!(coupon_rate, 3.25, epsilon=1.0e-6);
    }
}