mod data;
use std::collections::HashSet;

type Steps = i32;

#[derive(Clone, Debug)]
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

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
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
    
    fn approach(&mut self, position: &Position) {
        let delta_x = position.x - self.x;
        if delta_x > 0 {
            self.x += 1;
        } else if delta_x < 0 {
            self.x -= 1;
        }
        let delta_y = position.y - self.y;
        if delta_y > 0 {
            self.y += 1;
        } else if delta_y < 0 {
            self.y -= 1;
        }
    }
}

struct RopeSimulator {
    knots: Vec<Position>,
    visited_tail_positions: HashSet<Position>,
}

impl RopeSimulator {
    fn new(size: usize) -> Self {
        assert!(size >= 2);
        Self {
            knots: vec![Position::default(); size],
            visited_tail_positions: [Position::default()].into_iter().collect(), // tail is at the origin
        }
    }

    fn pull_rope(&mut self, motion: &Motion) {
        let steps = motion.steps();
        for _ in 0..steps {
            let head = self.knots.iter_mut().next().unwrap();
            head.step(motion);
            let knot_count = self.knots.len();
            for i in 1..knot_count {
                // cannot have mutable and immutable references of the same vec
                let lead = self.knots[i - 1].clone();
                let knot = &mut self.knots[i];
                if Self::knots_too_far_away(knot, &lead) {
                    knot.approach(&lead);
                    if Self::knots_too_far_away(knot, &lead) {
                        panic!("{knot:?} is too far from {lead:?}; step {i}, {motion:?}");
                    }
                }
            }
            self.visited_tail_positions
                .insert(self.knots.last().unwrap().clone());
        }
    }

    fn knots_too_far_away(knot1: &Position, knot2: &Position) -> bool {
        (knot1.x - knot2.x).abs() > 1 || (knot1.y - knot2.y).abs() > 1
    }
}

fn simulate_rope(size: usize, motions: &[Motion]) -> usize {
    let mut simulator = RopeSimulator::new(size);
    for motion in motions {
        simulator.pull_rope(motion);
    }
    simulator.visited_tail_positions.len()
}

fn calculate_solution(motions: &str) -> (usize, usize) {
    let motions = motions.lines().map(Motion::from).collect::<Vec<_>>();
    (simulate_rope(2, &motions), simulate_rope(10, &motions))
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::MOTIONS));
}
