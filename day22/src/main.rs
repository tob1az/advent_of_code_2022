mod data;

use regex::RegexBuilder;

type Number = usize;

#[derive(Debug)]
enum Step {
    Move(Number),
    TurnRight,
    TurnLeft,
}

type Path = Vec<Step>;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }
}

#[derive(Debug)]
enum Tile {
    None,
    Open,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::None,
            '.' => Self::Open,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    fn defined(&self) -> bool {
        !matches!(self, Tile::None)
    }
    fn vacant(&self) -> bool {
        matches!(self, Tile::Open)
    }
}

type Map = Vec<Vec<Tile>>;

fn parse_input(input: &str) -> (Path, Map) {
    let path_regex = RegexBuilder::new("\\d+|R|L").build().unwrap();
    let path = path_regex
        .captures_iter(input.lines().last().unwrap())
        .map(|c| match &c[0] {
            "L" => Step::TurnLeft,
            "R" => Step::TurnRight,
            distance => Step::Move(distance.parse().unwrap()),
        })
        .collect();

    let map = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().map(Tile::from).collect())
        .collect();

    (path, map)
}

struct Solver {
    path: Path,
    map: Map,
}

impl Solver {
    fn new(path: Path, map: Map) -> Self {
        Self { path, map }
    }

    fn determine_final_password(&self) -> Number {
        let mut row_number = 0;
        let mut column_number = self.map[0].iter().position(Tile::defined).unwrap();
        let mut direction = Direction::Right;
        for step in &self.path {
            match step {
                Step::TurnLeft => {
                    direction = direction.turn_left();
                }
                Step::TurnRight => {
                    direction = direction.turn_right();
                }
                Step::Move(x) => {
                    for _ in 0..*x {
                        if !self.step(&mut row_number, &mut column_number, direction) {
                            break;
                        }
                    }
                }
            }
        }
        1000 * (row_number + 1) + 4 * (column_number + 1) + direction as Number
    }

    fn step(
        &self,
        row_number: &mut usize,
        column_number: &mut usize,
        direction: Direction,
    ) -> bool {
        let row = &self.map[*row_number];

        match direction {
            Direction::Right => {
                let next_column = if *column_number + 1 < row.len() {
                    *column_number + 1
                } else {
                    row.iter().position(Tile::defined).unwrap()
                };

                if row[next_column].vacant() {
                    *column_number = next_column;
                } else {
                    return false;
                }
            }
            Direction::Left => {
                let next_column = if *column_number > 0 && row[*column_number - 1].defined() {
                    *column_number - 1
                } else {
                    row.len() - 1
                };

                if row[next_column].vacant() {
                    *column_number = next_column;
                } else {
                    return false;
                }
            }
            Direction::Down => {
                let next_row = if *row_number + 1 < self.map.len()
                    && self.map[*row_number + 1].len() > *column_number
                    && self.map[*row_number + 1][*column_number].defined()
                {
                    *row_number + 1
                } else {
                    let rev_row = self.map.len() - *row_number - 1;
                    // go up to find where map still defined
                    let height = self
                        .map
                        .iter()
                        .rev()
                        .skip(rev_row)
                        .take_while(|r| r.len() > *column_number && r[*column_number].defined())
                        .count();
                    *row_number + 1 - height
                };

                if self.map[next_row][*column_number].vacant() {
                    *row_number = next_row;
                } else {
                    return false;
                }
            }
            Direction::Up => {
                let next_row = if *row_number > 0
                    && self.map[*row_number - 1].len() > *column_number
                    && self.map[*row_number - 1][*column_number].defined()
                {
                    *row_number - 1
                } else {
                    let height = self
                        .map
                        .iter()
                        .skip(*row_number)
                        .take_while(|r| r.len() > *column_number && r[*column_number].defined())
                        .count();
                    *row_number + height - 1
                };

                if self.map[next_row][*column_number].vacant() {
                    *row_number = next_row;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

fn calculate_solution(input: &str) -> Number {
    let (path, map) = parse_input(input);
    Solver::new(path, map).determine_final_password()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reference_case_part_1() {
        assert_eq!(calculate_solution(data::TEST_INPUT), 6032);
    }
}
