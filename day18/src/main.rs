use std::collections::HashSet;

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
    let cubes: HashSet<&Cube> = HashSet::from_iter(cubes.iter());
    for cube in &cubes {
        for neighbor in cube.neighbors() {
            if !cubes.contains(&neighbor) {
                surface_sides.push(neighbor);
            }
        }
    }
    surface_sides.len()
}

fn point_in_box(point: &Point, min: &Point, max: &Point) -> bool {
    point.x >= min.x
        && point.x <= max.x
        && point.y >= min.y
        && point.y <= max.y
        && point.z >= min.z
        && point.z <= max.z
}

fn count_exterior_sides(cubes: &[Cube]) -> usize {
    let cubes: HashSet<&Cube> = HashSet::from_iter(cubes.iter());
    let mut queue = vec![Cube::new(Point::new(-1, -1, -1))];
    let mut visited = HashSet::new();
    // estimated bounds of the volume
    let min = Point::new(-1, -1, -1);
    let max = Point::new(25, 25, 25);
    while let Some(cube) = queue.pop() {
        for n in cube.neighbors() {
            if !cubes.contains(&n) && !visited.contains(&n) && point_in_box(&n.origin, &min, &max){
                queue.push(n);
            }
        }
        visited.insert(cube);
    }
    let mut exterior = 0;
    for cube in &cubes {
        for n in cube.neighbors() {
            if visited.contains(&n) {
                exterior += 1;
            }
        }
    }
    exterior
}

fn calculate_solution(cubes: &str) -> (usize, usize) {
    let cubes = parse_cubes(cubes);
    (count_surface_sides(&cubes), count_exterior_sides(&cubes))
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
    fn larger_set() {
        let cubes = parse_cubes(data::LARGER_TEST_SAMPLE);
        assert_eq!(count_surface_sides(&cubes), 64);
    }

    #[test]
    fn larger_set_exterior_only() {
        let cubes = parse_cubes(data::LARGER_TEST_SAMPLE);
        assert_eq!(count_exterior_sides(&cubes), 58);
    }
}
