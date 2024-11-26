use common::HftTimeseries;

fn main() -> anyhow::Result<()> {
    let ts1 = HftTimeseries::new(
        vec![1.0, 2.0, 3.0],
        vec![1, 2, 3],
    )?;

    let ts2 = HftTimeseries::new(
        vec![2.0, 3.0, 4.0],
        vec![2, 3, 4],
    )?;

    let ts = ts1 + ts2;

    println!("{:?}", ts);

    Ok(())
}