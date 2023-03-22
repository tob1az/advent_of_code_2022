mod data;
mod signal;

use itertools::Itertools;
use signal::Packet;

fn calculate_solution(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if c.filter_map(|p| Packet::parse(&p))
                .tuple_windows()
                .all(|(l, r)| l < r)
            {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}
