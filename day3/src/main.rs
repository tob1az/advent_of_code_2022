mod data;

use std::collections::HashSet;

type Priority = u32;
type Item = char;

#[derive(Debug)]
struct Rucksack<'a> {
    compartments: [&'a str; 2],
}

fn parse_rucksacks<'a>(rucksacks: &'a [&'a str]) -> Vec<Rucksack<'a>> {
    rucksacks
        .iter()
        .map(|r| {
            debug_assert!(r.len() % 2 == 0);
            let (first_compartment, second_compartment) = r.split_at(r.len() / 2);
            Rucksack {
                compartments: [first_compartment, second_compartment],
            }
        })
        .collect::<Vec<Rucksack<'a>>>()
}

fn prioritize(item: Item) -> Priority {
    if item.is_ascii_lowercase() {
        const BASE: Priority = 'a' as Priority;
        item as Priority - BASE + 1
    } else if item.is_ascii_uppercase() {
        const BASE: Priority = 'A' as Priority;
        item as Priority - BASE + 27
    } else {
        panic!("Unsupported item {item}")
    }
}

fn calculate_solution(rucksacks: &[&str]) -> Priority {
    let rucksacks = parse_rucksacks(rucksacks);
    rucksacks
        .iter()
        .map(|rucksack| {
            let items0 = rucksack.compartments[0].chars().collect::<HashSet<_>>();
            let items1 = rucksack.compartments[1].chars().collect::<HashSet<_>>();
            let duplicates = items0.intersection(&items1).collect::<Vec<_>>();
            debug_assert!(duplicates.len() == 1);
            prioritize(*duplicates.into_iter().next().unwrap())
        })
        .sum()
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::RUCKSACKS));
}
