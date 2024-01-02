use std::collections::HashSet;

mod data;

type Number = u32;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: Number,
    y: Number,
}

impl Point {
    fn shift(&self, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::Down => Some(Point {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::Up => {
                if self.y > 0 {
                    Some(Point {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    Some(Point {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::Right => Some(Point {
                x: self.x + 1,
                y: self.y,
            }),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(u8)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

type Blizzards = Vec<(Point, Direction)>;

struct Solver {
    blizzards: Blizzards,
    width: Number,
    height: Number,
    entry: Point,
    exit: Point,
}

impl Solver {
    fn solve(&self) -> Number {
        let mut positions = HashSet::from_iter([self.entry.clone()]);
        let mut blizzards = self.blizzards.clone();
        let mut minutes = 1;
        loop {
            let mut new_positions = HashSet::with_capacity(positions.len());
            let max_x = self.width - 1;
            let max_y = self.height - 1;
            self.move_blizzards(&mut blizzards);
            for position in &positions {
                for direction in [
                    Some(Direction::Down),
                    Some(Direction::Right),
                    Some(Direction::Up),
                    Some(Direction::Left),
                    None,
                ] {
                    if let Some(new_position) = match direction {
                        Some(d) => position.shift(&d),
                        None => Some(position.clone()),
                    } {
                        if new_position == self.exit {
                            dbg!(positions.len());
                            return minutes;
                        }

                        if new_position != self.entry
                            && (new_position.x == 0
                                || new_position.y == 0
                                || new_position.x == max_x
                                || new_position.y == max_y)
                        {
                            continue;
                        }
                        if !blizzards.iter().any(|(p, _)| *p == new_position) {
                            new_positions.insert(new_position);
                        }
                    }
                }
            }
            if positions.is_empty() {
                panic!("lost track at {minutes}");
            }
            positions = new_positions;
            minutes += 1;
        }
    }

    fn move_blizzards(&self, blizzards: &mut Blizzards) {
        let new = blizzards
            .iter_mut()
            .filter_map(|(point, direction)| {
                point.shift(direction).map(|mut shifted| {
                    // wrap up as needed
                    if shifted.x == 0 {
                        shifted.x = self.width - 2;
                    } else if shifted.x == self.width - 1 {
                        shifted.x = 1;
                    } else if shifted.y == 0 {
                        shifted.y = self.height - 2;
                    } else if shifted.y == self.height - 1 {
                        shifted.y = 1;
                    }
                    (shifted, direction.clone())
                })
            })
            .collect();
        *blizzards = new;
    }
}

fn parse_input(input: &str) -> Solver {
    let height = input.lines().count() as Number;
    let first_line = input.lines().next().unwrap();
    let width = first_line.len() as Number;
    let entry = Point {
        x: first_line.find('.').unwrap() as Number,
        y: 0,
    };
    let last_line = input.lines().last().unwrap();
    let exit = Point {
        x: last_line.find('.').unwrap() as Number,
        y: (height - 1) as Number,
    };
    let blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if let Ok(direction) = Direction::try_from(c) {
                        Some((
                            Point {
                                x: x as Number,
                                y: y as Number,
                            },
                            direction,
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Solver {
        blizzards,
        width,
        height,
        entry,
        exit,
    }
}

fn calculate_solution(input: &str) -> Number {
    let solver = parse_input(input);
    solver.solve()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reference_case_part_1() {
        assert_eq!(calculate_solution(data::TEST_INPUT), 18);
    }
}
