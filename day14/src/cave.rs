use itertools::Itertools;
use std::fmt;

#[derive(Clone, Debug)]
enum Tile {
    Empty,
    Rock,
    Sand,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ".",
                Tile::Rock => "#",
                Tile::Sand => "o",
            }
        )
    }
}

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

const INGRESS_POINT: Point = Point { x: 500, y: 0 };

pub struct Cave {
    tiles: Vec<Vec<Tile>>,
    min_x: usize,
    max_x: usize,
    max_y: usize,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} - {}", self.min_x, self.max_x)?;
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        write!(f, "{}", self.max_y)
    }
}

pub fn parse_cave(rock_scan: &str) -> Cave {
    let rock_traces = rock_scan
        .lines()
        .map(|trace| {
            trace
                .split(" -> ")
                .map(|coord| {
                    coord
                        .split_once(',')
                        .map(|(x, y)| Point {
                            x: x.parse::<usize>().unwrap(),
                            y: y.parse::<usize>().unwrap(),
                        })
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let min_x = rock_traces.iter().flatten().min_by_key(|t| t.x).unwrap().x;
    let max_x = rock_traces.iter().flatten().max_by_key(|t| t.x).unwrap().x;
    let max_y = rock_traces.iter().flatten().max_by_key(|t| t.y).unwrap().y;
    let width = max_x - min_x + 1;
    let mut tiles = (0..=max_y)
        .map(|_| vec![Tile::Empty; width])
        .collect::<Vec<_>>();
    for trace in rock_traces.iter() {
        for (a, b) in trace.iter().tuple_windows() {
            let (from_x, to_x) = if a.x < b.x {
                (a.x - min_x, b.x - min_x)
            } else {
                (b.x - min_x, a.x - min_x)
            };
            let (from_y, to_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
            assert!(!(from_x != to_x && from_y != to_y));
            if from_x != to_x {
                for x in from_x..=to_x {
                    tiles[from_y][x] = Tile::Rock;
                }
            } else {
                for y in from_y..=to_y {
                    tiles[y][from_x] = Tile::Rock;
                }
            }
        }
    }
    Cave {
        tiles,
        min_x,
        max_x,
        max_y,
    }
}

impl Cave {
    pub fn drop_sand(&mut self) -> bool {
        let mut x = INGRESS_POINT.x;
        for y in INGRESS_POINT.y..=self.max_y {
            let next = &self.tiles[y][x - self.min_x];
            if matches!(next, Tile::Empty) {
                continue;
            }
            if x == self.min_x || y == 0 {
                return false;
            }
            x -= 1;
            let next = &self.tiles[y][x - self.min_x];
            if matches!(next, Tile::Empty) {
                continue;
            }
            x += 2;
            if x > self.max_x {
                return false;
            }
            let next = &self.tiles[y][x - self.min_x];
            if matches!(next, Tile::Empty) {
                continue;
            }
            let settled = &mut self.tiles[y - 1][x - 1 - self.min_x];
            *settled = Tile::Sand;
            return true;
        }
        false
    }

    pub fn max_y(&self) -> usize {
        self.max_y
    }
}
