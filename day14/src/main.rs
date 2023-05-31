mod cave;
mod data;

fn count_grains(mut cave: cave::Cave) -> usize {
    let mut num_sand_units = 0;
    while cave.drop_sand() {
        num_sand_units += 1;
    }
    println!("full: {cave}");
    num_sand_units
}

fn calculate_solution(rock_scan: &str) -> (usize, usize) {
    let bottomless_cave = cave::parse_cave(rock_scan);
    let bottom_y = bottomless_cave.max_y() + 2;
    println!("cave: {bottomless_cave}");
    let scan_with_bottom = format!("{}\n0,{bottom_y} -> 1000,{bottom_y}", rock_scan);
    (
        count_grains(bottomless_cave),
        count_grains(cave::parse_cave(&scan_with_bottom)),
    )
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::ROCK_SCAN));
}
