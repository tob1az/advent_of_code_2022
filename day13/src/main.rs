mod data;
mod signal;

use itertools::Itertools;
use signal::Packet;

fn sorted_indices_sum(packets: &[Vec<Packet>]) -> usize {
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p.iter().tuple_windows().all(|(l, r)| l < r) {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn divider(number: u32) -> Packet {
    Packet::Nested(vec![Packet::Nested(vec![Packet::Digit(number)])])
}

fn decoder_key(packets: &[Vec<Packet>]) -> usize {
    let dividers = vec![divider(2), divider(6)];
    dividers
        .iter()
        .chain(packets.iter().flatten())
        .sorted()
        .enumerate()
        .filter_map(|(i, p)| {
            if *p == dividers[0] || *p == dividers[1] {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

fn calculate_solution(input: &str) -> (usize, usize) {
    let packets = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| c.filter_map(Packet::parse).collect_vec())
        .collect_vec();

    (sorted_indices_sum(&packets), decoder_key(&packets))
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}
