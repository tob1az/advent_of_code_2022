mod data;
use std::collections::HashSet;

type Steps = i32;

enum Motion {
    Up(Steps),
    Down(Steps),
    Left(Steps),
    Right(Steps),
}

fn parse_steps(steps: &str) -> Steps {
    steps.parse::<Steps>().expect("Integer steps")
}

impl From<&str> for Motion {
    fn from(motion: &str) -> Self {
        match motion.split_once(' ') {
            Some(("U", steps)) => Motion::Up(parse_steps(steps)),
            Some(("D", steps)) => Motion::Down(parse_steps(steps)),
            Some(("L", steps)) => Motion::Left(parse_steps(steps)),
            Some(("R", steps)) => Motion::Right(parse_steps(steps)),
            _ => panic!("Unknown direction"),
        }
    }
}

type Coordinate = i32;

#[derive(Default, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: Coordinate,
    y: Coordinate,
}

impl Position {
    fn shift(&mut self, motion: &Motion) {
        use Motion::*;
        match motion {
            Up(s) => self.y += s,
            Down(s) => self.y -= s,
            Left(s) => self.x -= s,
            Right(s) => self.x += s
        }
    }
}

#[derive(Default)]
struct RopeSimulator {
    head: Position,
    tail: Position,
    visited_tail_positions: HashSet<Position>,
}

impl RopeSimulator {
    fn pull_rope(&mut self, motion: &Motion) {
        self.head.shift(motion);
        // TODO: implement correct tail motion
        self.tail.shift(motion);
        self.visited_tail_positions.insert(self.tail.clone());
    }
}

fn calculate_solution(motions: &str) -> usize {
    let motions = motions.lines().map(|l| Motion::from(l)).collect::<Vec<_>>();
    let mut simulator = RopeSimulator::default();
    for motion in motions.into_iter() {
        simulator.pull_rope(&motion);
    }
    simulator.visited_tail_positions.len()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::MOTIONS));
}
