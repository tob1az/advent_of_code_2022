mod common;
mod cube;
mod data;

use common::{Direction, MovingPoint, Number};
use regex::RegexBuilder;

#[derive(Debug)]
enum Step {
    Move(Number),
    TurnRight,
    TurnLeft,
}

type Path = Vec<Step>;

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

enum Mode {
    Flat,
    Cubic(cube::Configuration),
}

impl Solver {
    fn new(path: Path, map: Map) -> Self {
        Self { path, map }
    }

    fn determine_final_password(&self, mode: Mode) -> Number {
        let mut point = MovingPoint {
            column: self.map[0].iter().position(Tile::defined).unwrap() as Number,
            row: 0,
            direction: Direction::Right,
        };
        for step in &self.path {
            match step {
                Step::TurnLeft => {
                    point.direction = point.direction.turn_left();
                }
                Step::TurnRight => {
                    point.direction = point.direction.turn_right();
                }
                Step::Move(x) => {
                    for _ in 0..*x {
                        if !self.step(&mut point, &mode) {
                            break;
                        }
                    }
                }
            }
        }
        1000 * (point.row + 1) + 4 * (point.column + 1) + point.direction as Number
    }

    fn step(&self, point: &mut MovingPoint, mode: &Mode) -> bool {
        let next_point = self.move_point(point, mode);
        if self.map[next_point.row as usize][next_point.column as usize].vacant() {
            *point = next_point;
            true
        } else {
            false
        }
    }

    fn move_point(&self, point: &MovingPoint, mode: &Mode) -> MovingPoint {
        let column_number = point.column as usize;
        let row_number = point.row as usize;
        match point.direction {
            Direction::Right => {
                if column_number + 1 < self.map[row_number].len() {
                    MovingPoint {
                        column: point.column + 1,
                        row: point.row,
                        direction: point.direction,
                    }
                } else {
                    match mode {
                        Mode::Flat => MovingPoint {
                            column: self.map[row_number].iter().position(Tile::defined).unwrap()
                                as Number,
                            row: point.row,
                            direction: point.direction,
                        },
                        Mode::Cubic(config) => {
                            config.transform_point(point, |p| self.is_correct_point(p))
                        }
                    }
                }
            }
            Direction::Left => {
                if column_number > 0 && self.map[row_number][column_number - 1].defined() {
                    MovingPoint {
                        column: point.column - 1,
                        row: point.row,
                        direction: point.direction,
                    }
                } else {
                    match mode {
                        Mode::Flat => MovingPoint {
                            column: self.map[row_number].len() as Number - 1,
                            row: point.row,
                            direction: point.direction,
                        },
                        Mode::Cubic(config) => {
                            config.transform_point(point, |p| self.is_correct_point(p))
                        }
                    }
                }
            }
            Direction::Down => {
                if row_number + 1 < self.map.len()
                    && self.map[row_number + 1].len() > column_number
                    && self.map[row_number + 1][column_number].defined()
                {
                    MovingPoint {
                        column: point.column,
                        row: point.row + 1,
                        direction: point.direction,
                    }
                } else {
                    match mode {
                        Mode::Flat => {
                            let rev_row = self.map.len() - row_number - 1;
                            // go up to find where map still defined
                            let height = self
                                .map
                                .iter()
                                .rev()
                                .skip(rev_row)
                                .take_while(|r| {
                                    r.len() > column_number && r[column_number].defined()
                                })
                                .count() as Number;

                            MovingPoint {
                                column: point.column,
                                row: point.row + 1 - height,
                                direction: point.direction,
                            }
                        }
                        Mode::Cubic(config) => {
                            config.transform_point(point, |p| self.is_correct_point(p))
                        }
                    }
                }
            }
            Direction::Up => {
                if row_number > 0
                    && self.map[row_number - 1].len() > column_number
                    && self.map[row_number - 1][column_number].defined()
                {
                    MovingPoint {
                        column: point.column,
                        row: point.row - 1,
                        direction: point.direction,
                    }
                } else {
                    match mode {
                        Mode::Flat => {
                            // go up to find where map still defined
                            let height = self
                                .map
                                .iter()
                                .skip(row_number)
                                .take_while(|r| {
                                    r.len() > column_number && r[column_number].defined()
                                })
                                .count() as Number;

                            MovingPoint {
                                column: point.column,
                                row: point.row + height - 1,
                                direction: point.direction,
                            }
                        }
                        Mode::Cubic(config) => {
                            config.transform_point(point, |p| self.is_correct_point(p))
                        }
                    }
                }
            }
        }
    }

    fn is_correct_point(&self, point: &MovingPoint) -> bool {
        if point.row < 0 || point.column < 0 {
            return false;
        }
        if let Some(row) = self.map.get(point.row as usize) {
            if let Some(tile) = row.get(point.column as usize) {
                return tile.defined();
            }
        }
        false
    }
}

fn calculate_solution(input: &str) -> (Number, Number) {
    let (path, map) = parse_input(input);
    let solver = Solver::new(path, map);
    let cubic_mode = Mode::Cubic(cube::Configuration {
        side_length: data::CUBE_SIDE,
        edges: data::CUBE_EDGES.into(),
    });
    (
        solver.determine_final_password(Mode::Flat),
        solver.determine_final_password(cubic_mode),
    )
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reference_case_part_1() {
        let (path, map) = parse_input(data::TEST_INPUT);
        assert_eq!(
            Solver::new(path, map).determine_final_password(Mode::Flat),
            6032
        );
    }

    #[test]
    fn reference_case_part_2() {
        let mode = Mode::Cubic(cube::Configuration {
            side_length: data::TEST_CUBE_SIDE,
            edges: data::TEST_CUBE_EDGES.into(),
        });
        let (path, map) = parse_input(data::TEST_INPUT);
        assert_eq!(Solver::new(path, map).determine_final_password(mode), 5031);
    }
}
