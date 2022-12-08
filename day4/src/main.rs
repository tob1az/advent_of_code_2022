mod data;

use itertools::Itertools;

struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn contains(&self, range: &Range) -> bool {
        debug_assert!(range.from <= range.to);
        self.from <= range.from && self.to >= range.to
    }

    fn overlaps(&self, range: &Range) -> bool {
        debug_assert!(range.from <= range.to);
        !((self.from < range.from && self.to < range.from)
            || (self.from > range.to && self.to > range.to))
    }
}

type Pair = (Range, Range);

fn count_duplicates(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|pair| pair.0.contains(&pair.1) || pair.1.contains(&pair.0))
        .count()
}

fn count_overlaps(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|pair| pair.0.overlaps(&pair.1)).count()
}

fn parse_assignments(assignments: &str) -> Vec<Pair> {
    assignments
        .lines()
        .map(|line| {
            let pair = line.split(',').next_tuple::<(_, _)>().unwrap();
            let first_range = pair.0.split('-').next_tuple::<(_, _)>().unwrap();
            let second_range = pair.1.split('-').next_tuple::<(_, _)>().unwrap();
            (
                Range {
                    from: first_range.0.parse().unwrap(),
                    to: first_range.1.parse().unwrap(),
                },
                Range {
                    from: second_range.0.parse().unwrap(),
                    to: second_range.1.parse().unwrap(),
                },
            )
        })
        .collect()
}

fn calculate_solution(assignments: &str) -> (usize, usize) {
    let assignments = parse_assignments(assignments);

    (count_duplicates(&assignments), count_overlaps(&assignments))
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(data::SECTION_ASSIGNMENTS)
    );
}
