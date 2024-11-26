#[cfg(test)]
mod tests {
    use common::HftTimeseries;

    #[test]
    fn test_add() -> anyhow::Result<()> {
        let ts1 = HftTimeseries::new(
            vec![1.0, 2.0, 3.0],
            vec![1, 2, 3],
        )?;

        let ts2 = HftTimeseries::new(
            vec![2.0, 3.0, 4.0],
            vec![2, 3, 4],
        )?;

        let ts = ts1 + ts2;

        assert_eq!(ts.data, vec![4.0, 6.0, 7.0]);
        assert_eq!(ts.timestamps, vec![2, 3, 4]);

        Ok(())

    }
}