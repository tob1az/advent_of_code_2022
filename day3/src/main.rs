mod data;

use itertools::Itertools;
use std::collections::HashSet;

type Priority = u32;
type Item = char;
type Items = HashSet<Item>;

#[derive(Debug)]
struct Rucksack {
    items: Items,
    first_compartment: Items,
    second_compartment: Items,
}

impl Rucksack {
    fn new(items: &str) -> Self {
        debug_assert!(items.len() % 2 == 0);
        let (first, second) = items.split_at(items.len() / 2);
        Self {
            items: items.chars().collect::<Items>(),
            first_compartment: first.chars().collect::<Items>(),
            second_compartment: second.chars().collect::<Items>(),
        }
    }
}

fn parse_rucksacks(rucksacks: &[&str]) -> Vec<Rucksack> {
    rucksacks
        .iter()
        .map(|r| Rucksack::new(*r))
        .collect::<Vec<_>>()
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

fn calculate_solution(rucksacks: &[&str]) -> (Priority, Priority) {
    let rucksacks = parse_rucksacks(rucksacks);
    let answer1 = rucksacks
        .iter()
        .map(|rucksack| {
            let duplicates = rucksack
                .first_compartment
                .intersection(&rucksack.second_compartment)
                .cloned()
                .collect::<Vec<_>>();
            debug_assert!(duplicates.len() == 1);
            prioritize(duplicates[0])
        })
        .sum();

    debug_assert!(rucksacks.len() % 3 == 0);
    let answer2 = rucksacks
        .into_iter()
        .tuples()
        .map(|(first, second, third)| {
            let shared = first
                .items
                .intersection(&second.items)
                .cloned()
                .collect::<HashSet<_>>();
            debug_assert!(shared.len() >= 1);
            let shared = third.items.intersection(&shared).collect::<Vec<_>>();
            debug_assert!(shared.len() == 1);
            prioritize(*shared[0])
        })
        .sum();

    (answer1, answer2)
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::RUCKSACKS));
}
