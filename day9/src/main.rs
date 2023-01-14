mod data;
use std::collections::HashSet;

type Steps = i32;

enum Motion {
    Up(Steps),
    Down(Steps),
    Left(Steps),
    Right(Steps),
}

impl Motion {
    fn steps(&self) -> Steps {
        use Motion::*;
        match *self {
            Up(s) => s,
            Down(s) => s,
            Left(s) => s,
            Right(s) => s,
        }
    }
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
    fn step(&mut self, motion: &Motion) {
        use Motion::*;
        match motion {
            Up(_) => self.y += 1,
            Down(_) => self.y -= 1,
            Left(_) => self.x -= 1,
            Right(_) => self.x += 1,
        }
    }
}

struct RopeSimulator {
    head: Position,
    tail: Position,
    visited_tail_positions: HashSet<Position>,
}

impl RopeSimulator {
    fn new() -> Self {
        Self {
            head: Position::default(),
            tail: Position::default(),
            visited_tail_positions: [Position::default()].into_iter().collect(), // tail is at the origin
        }
    }

    fn pull_rope(&mut self, motion: &Motion) {
        let steps = motion.steps();
        for _ in 0..steps {
            self.head.step(motion);
            if self.tail_too_far_away() {
                self.pull_tail(motion);
                self.visited_tail_positions.insert(self.tail.clone());
            }
        }
    }

    fn tail_too_far_away(&self) -> bool {
        (self.head.x - self.tail.x).abs() > 1 || (self.head.y - self.tail.y).abs() > 1
    }

    fn pull_tail(&mut self, motion: &Motion) {
        if (self.head.x - self.tail.x).abs() > 1 || (self.head.y - self.tail.y).abs() > 1 {
            use Motion::*;
            match motion {
                Up(_) => {
                    self.tail = Position {
                        x: self.head.x,
                        y: self.head.y - 1,
                    }
                }
                Down(_) => {
                    self.tail = Position {
                        x: self.head.x,
                        y: self.head.y + 1,
                    }
                }
                Left(_) => {
                    self.tail = Position {
                        x: self.head.x + 1,
                        y: self.head.y,
                    }
                }
                Right(_) => {
                    self.tail = Position {
                        x: self.head.x - 1,
                        y: self.head.y,
                    }
                }
            }
        }
    }
}

fn calculate_solution(motions: &str) -> usize {
    let motions = motions.lines().map(|l| Motion::from(l)).collect::<Vec<_>>();
    let mut simulator = RopeSimulator::new();
    for motion in motions.into_iter() {
        simulator.pull_rope(&motion);
    }
    simulator.visited_tail_positions.len()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::MOTIONS));
}
