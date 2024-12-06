use crate::payload_field::PayloadField;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_json() -> anyhow::Result<()> {
        let csv_path = "data/multiasset_db.csv";
        let _fields = PayloadField::load_from_csv(csv_path)
            .expect("Failed to load CSV file");

        // ... 나머지 코드 ...

        Ok(())
    }
}