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
}

type Pair = (Range, Range);

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

fn calculate_solution(assignments: &str) -> usize {
    parse_assignments(assignments)
        .iter()
        .filter(|pair| pair.0.contains(&pair.1) || pair.1.contains(&pair.0))
        .count()
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(data::SECTION_ASSIGNMENTS)
    );
}
