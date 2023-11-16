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

#[derive(Debug, Hash, PartialEq, Eq)]
struct Cube {
    origin: Point,
}

impl Cube {
    fn new(origin: Point) -> Self {
        Self { origin }
    }
    fn neighbors(&self) -> Vec<Cube> {
        vec![
            Cube::new(self.origin.replicate(1, 0, 0)),
            Cube::new(self.origin.replicate(-1, 0, 0)),
            Cube::new(self.origin.replicate(0, 1, 0)),
            Cube::new(self.origin.replicate(0, -1, 0)),
            Cube::new(self.origin.replicate(0, 0, 1)),
            Cube::new(self.origin.replicate(0, 0, -1)),
        ]
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
            Cube::new(Point::new(x, y, z))
        })
        .collect()
}

fn count_surface_sides(cubes: &[Cube]) -> usize {
    let mut surface_sides = Vec::new();
    for cube in cubes {
        for neighbor in cube.neighbors() {
            if !cubes.contains(&neighbor) {
                surface_sides.push(neighbor);
            }
        }
    }
    surface_sides.len()
}

fn calculate_solution(cubes: &str) -> usize {
    let cubes = parse_cubes(cubes);
    count_surface_sides(&cubes)
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_case() {
        let cubes = parse_cubes(data::TWO_ADJACENT);
        assert_eq!(count_surface_sides(&cubes), 10);
    }
    #[test]
    fn larger_set_case() {
        let cubes = parse_cubes(data::LARGER_TEST_SAMPLE);
        assert_eq!(count_surface_sides(&cubes), 64);
    }
}
