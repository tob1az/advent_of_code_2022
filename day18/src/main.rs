use std::collections::HashMap;

mod data;

type Distance = i64;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: Distance,
    y: Distance,
    z: Distance,
}

impl Point {
    fn new(x: Distance, y: Distance, z: Distance) -> Self {
        Self { x, y, z }
    }
    fn replicate(&self, dx: Distance, dy: Distance, dz: Distance) -> Self {
        Self::new(self.x + dx, self.y + dy, self.z + dz)
    }
}

#[derive(Debug, Hash)]
struct Side {
    from: Point,
    to: Point,
}

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.from == other.to && self.to == other.from)
    }
}

impl Eq for Side {}

impl Side {
    fn new(from: Point, to: Point) -> Self {
        Self { from, to }
    }
}

#[derive(Debug)]
struct Cube {
    sides: [Side; 6],
}

impl Cube {
    fn from_origin(origin: Point) -> Self {
        let opposite = origin.replicate(1, 1, 1);
        Self {
            sides: [
                Side::new(origin, origin.replicate(1, 1, 0)),
                Side::new(origin, origin.replicate(0, 1, 1)),
                Side::new(origin, origin.replicate(1, 0, 1)),
                Side::new(origin.replicate(1, 0, 0), opposite),
                Side::new(origin.replicate(0, 1, 0), opposite),
                Side::new(origin.replicate(0, 0, 1), opposite),
            ],
        }
    }
}

fn parse_cubes(cubes: &str) -> Vec<Cube> {
    cubes
        .lines()
        .map(|l| {
            let mut iter = l.split(',');
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            let z = iter.next().unwrap().parse().unwrap();
            Cube::from_origin(Point::new(x, y, z))
        })
        .collect()
}

fn calculate_solution(cubes: &str) -> usize {
    let cubes = parse_cubes(cubes);
    let mut unique_sides = HashMap::new();
    for cube in &cubes {
        for side in &cube.sides {
            unique_sides
                .entry(side)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
    unique_sides.iter().filter(|(_, v)| **v == 1).count()
}

fn main() {
    println!("Solution {}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(calculate_solution(data::TWO_ADJACENT), 10);
    }
    #[test]
    fn larger_set_case() {
        assert_eq!(calculate_solution(data::LARGER_TEST_SAMPLE), 64);
    }
}
