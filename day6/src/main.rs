mod data;

use itertools::Itertools;

fn calculate_solution(data_stream: &str) -> usize {
    let p = data_stream
        .as_bytes()
        .iter()
        .enumerate()
        .tuple_windows()
        .skip_while(|(a, b, c, d)| {
            !(a.1 != b.1 && a.1 != c.1 && a.1 != d.1 && b.1 != c.1 && b.1 != d.1 && c.1 != d.1)
        })
        .next()
        .unwrap();
    println!("{p:?}");
    p.0 .0 + 4 // 1-based index of the first packet byte after the 4-byte header
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::DATASTREAM));
}
