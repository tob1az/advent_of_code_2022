mod data;
mod stack;

use regex::Regex;
use stack::Stack;

type Crate = char;
type CrateStack = stack::Stack<Crate>;

fn parse_crate_stacks(crates: &str) -> Vec<CrateStack> {
    let stack_count = (crates
        .lines()
        .rev()
        .nth(1)
        .expect("non-empty crates list")
        .len()
        + 1)
        / 4_usize;
    let mut stacks = Vec::with_capacity(stack_count);
    stacks.resize(stack_count, Stack::default());

    for row in crates.lines().rev().skip(1) {
        for (i, c) in row.chars().enumerate() {
            if c == ' ' || c == '[' || c == ']' {
                continue;
            }
            debug_assert!(i > 0);
            let stack_index = (i - 1) / 4;
            stacks[stack_index].push(c);
        }
    }
    stacks
}

#[derive(Debug)]
struct Move {
    crates_to_move: usize,
    from_index: usize,
    to_index: usize,
}

fn parse_moves(moves: &str) -> Vec<Move> {
    let mut parsed = vec![];
    let re = Regex::new("move (.+) from (.+) to (.+)\n").expect("Regex is valid");
    for captures in re.captures_iter(moves) {
        parsed.push(Move {
            crates_to_move: captures[1].parse::<usize>().unwrap(),
            from_index: captures[2].parse::<usize>().unwrap() - 1,
            to_index: captures[3].parse::<usize>().unwrap() - 1,
        })
    }
    parsed
}

fn calculate_solution(crates: &str, moves: &str) -> String {
    let mut stacks = parse_crate_stacks(crates);
    for (move_no, crate_move) in parse_moves(moves).into_iter().enumerate() {
        for _i in 0..crate_move.crates_to_move {
            if let Some(c) = stacks[crate_move.from_index].pop() {
                stacks[crate_move.to_index].push(c);
            } else {
                panic!("Wrong move #{move_no} {crate_move:?}");
            }
        }
    }
    // pick top crates
    stacks
        .into_iter()
        .filter_map(|s| {
            if let Some(&c) = s.top() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<String>()
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(data::CRATES, data::MOVES)
    );
}
