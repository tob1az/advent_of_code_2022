mod data;

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

fn calculate_solution(motions: &str) -> usize {
    let _motions = motions.lines().map(|l| Motion::from(l)).collect::<Vec<_>>();
    0
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::MOTIONS));
}
