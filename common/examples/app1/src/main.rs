//use common::HftTimeseries;
use common::packet::packet_extractor::PacketExtractor;

fn main() -> anyhow::Result<()> {
    let input_file = "D:/DATA/koscom_udp_2024-09-27.pcap";
    let output_file = "D:/DATA/koscom_udp_2024-09-27_filtered.pcap";
    let header_filter = Some(vec![
        String::from("B606F"),
        String::from("B601K"),
        String::from("A301K"),
        String::from("A306F"),
        String::from("G706F"),
        String::from("G701K"),
        String::from("H106F"),
        String::from("H201F"),
        ]);

    let packet_extractor = PacketExtractor::new(
        input_file.to_string(),
        output_file.to_string(),
        header_filter,
    );

    packet_extractor.filter_packets_with_header();

    Ok(())
}


/*
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

*/