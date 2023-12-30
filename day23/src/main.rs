use std::collections::{HashMap, HashSet};

mod data;

type Number = isize;

#[derive(Clone, Copy, Debug)]
enum Direction {
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE,
    E,
}

struct CardinalDirectionIter {
    direction: Direction,
}

fn cardinal_directions_from(direction: Direction) -> CardinalDirectionIter {
    assert!(matches!(
        direction,
        Direction::N | Direction::S | Direction::E | Direction::W
    ));
    CardinalDirectionIter { direction }
}

impl Iterator for CardinalDirectionIter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        let direction = self.direction;
        self.direction = match self.direction {
            Direction::N => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::E,
            Direction::E => Direction::N,
            _ => unreachable!("Only cardinal directions are accepted"),
        };
        Some(direction)
    }
}

struct CompassRoseIter {
    direction: Direction,
}

fn directions_from(start: Direction) -> CompassRoseIter {
    CompassRoseIter { direction: start }
}

impl Iterator for CompassRoseIter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        let direction = self.direction;
        self.direction = match self.direction {
            Direction::N => Direction::NE,
            Direction::NE => Direction::E,
            Direction::E => Direction::SE,
            Direction::SE => Direction::S,
            Direction::S => Direction::SW,
            Direction::SW => Direction::W,
            Direction::W => Direction::NW,
            Direction::NW => Direction::N,
        };
        Some(direction)
    }
}

impl DoubleEndedIterator for CompassRoseIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let direction = self.direction;
        self.direction = match self.direction {
            Direction::N => Direction::NW,
            Direction::NW => Direction::W,
            Direction::W => Direction::SW,
            Direction::SW => Direction::S,
            Direction::S => Direction::SE,
            Direction::SE => Direction::E,
            Direction::E => Direction::NE,
            Direction::NE => Direction::N,
        };
        Some(direction)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: Number,
    y: Number,
}

impl Point {
    fn go(&self, direction: Direction) -> Point {
        match direction {
            Direction::N => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::NE => Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::E => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::SE => Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::S => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::SW => Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::W => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::NW => Point {
                x: self.x - 1,
                y: self.y - 1,
            },
        }
    }
}

fn parse_elf_positions(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Point {
                            x: x as Number,
                            y: y as Number,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>()
}

struct Simulation {
    elf_positions: HashSet<Point>,
    start_from: Direction,
}

impl Simulation {
    fn round(&mut self) {
        // propose move
        let mut proposed_moves = HashMap::new();
        for position in &self.elf_positions {
            // if an elf is surrounded by empty space, don't move them
            if self.has_empty_tiles(position, Direction::N, 9) {
                continue;
            }
            // find first valid direction
            if let Some(proposed) = cardinal_directions_from(self.start_from)
                .take(4)
                .filter_map(|d| {
                    // start from NE, not N and so on
                    let start = directions_from(d).rev().nth(1).unwrap();
                    if self.has_empty_tiles(position, start, 3) {
                        Some(position.go(d))
                    } else {
                        None
                    }
                })
                .next()
            {
                proposed_moves
                    .entry(proposed)
                    .or_insert_with(Vec::new)
                    .push(position.clone());
            }
        }
        // eliminate conflicts
        proposed_moves.retain(|_, v| v.len() == 1);
        let to_delete = proposed_moves.values().flatten().collect::<HashSet<_>>();
        // retain elves who don't move
        self.elf_positions.retain(|p| !to_delete.contains(p));
        // add new elf positions
        self.elf_positions.extend(proposed_moves.keys().cloned());

        self.start_from = cardinal_directions_from(self.start_from).nth(1).unwrap();
    }

    fn has_empty_tiles(&self, point: &Point, start_from: Direction, position_count: usize) -> bool {
        directions_from(start_from)
            .take(position_count)
            .all(|d| !self.elf_positions.contains(&point.go(d)))
    }

    fn find_bounds(&self) -> (Point, Point) {
        let min_x = self.elf_positions.iter().min_by_key(|p| p.x).unwrap().x;
        let max_x = self.elf_positions.iter().max_by_key(|p| p.x).unwrap().x;
        let min_y = self.elf_positions.iter().min_by_key(|p| p.y).unwrap().y;
        let max_y = self.elf_positions.iter().max_by_key(|p| p.y).unwrap().y;
        (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
    }

    fn count_empty_tiles(&self) -> usize {
        let (min, max) = self.find_bounds();
        let mut count = 0;
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                if !self.elf_positions.contains(&Point { x, y }) {
                    count += 1;
                }
            }
        }
        count
    }

    fn print_tiles(&self) {
        let (min, max) = self.find_bounds();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if !self.elf_positions.contains(&Point { x, y }) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}

fn calculate_solution(input: &str) -> usize {
    let mut simulation = Simulation {
        elf_positions: parse_elf_positions(input),
        start_from: Direction::N,
    };
    simulation.print_tiles();
    for i in 0..10 {
        println!("round {i}");
        simulation.round();
        simulation.print_tiles();
    }
    simulation.count_empty_tiles()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reference_case_part_1() {
        assert_eq!(calculate_solution(data::TEST_INPUT), 110);
    }
}
