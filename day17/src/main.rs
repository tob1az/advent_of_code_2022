mod data;
mod tetris;

fn parse_moves(jet_pattern: &str) -> Vec<i32> {
    jet_pattern
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("Invalid character {c} in jet pattern"),
        })
        .collect()
}

fn calculate_solution(jet_pattern: &str) -> usize {
    let moves = parse_moves(jet_pattern);
    let mut sim = tetris::Simulation::new(7, 2, &moves);
    for _ in 0..2022 {
        sim.throw_next_shape();
    }
    sim.height() as usize
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::JET_PATTERN));
}
