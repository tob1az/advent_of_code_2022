mod data;
mod cave;

fn calculate_solution(rock_scan: &str) -> usize {
    let mut cave = cave::parse_cave(rock_scan);
    println!("cave: {cave}");
    let mut num_sand_units = 0;
    while cave.drop_sand() {
        num_sand_units += 1;
    }
    num_sand_units
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::ROCK_SCAN));
}
