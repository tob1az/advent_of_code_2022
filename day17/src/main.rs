mod data;
mod tetris;

fn parse_moves(jet_pattern: &str) -> Vec<i64> {
    jet_pattern
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("Invalid character {c} in jet pattern"),
        })
        .collect()
}

fn compute_rock_tower_height(moves: &[i64], total_throws: usize) -> usize {
    let mut sim = tetris::Simulation::new(7, 2, &moves);
    let mut skipped_throws = 0;
    let mut skipped_cycles = 0;
    let mut cycle_height = 0;
    let mut cycle_found = false;
    for shapes_thrown in 1..=total_throws {
        sim.throw_next_shape();
        if shapes_thrown % 1000 == 0 {
            dbg!(shapes_thrown);
        }
        if !cycle_found {
            if let Some(cycle) = sim.find_throw_cycle() {
                // fast forward
                cycle_height = (sim.height() - cycle.height) as usize;
                let throws_per_cycle = shapes_thrown - cycle.throw_count;
                skipped_cycles = (total_throws - shapes_thrown) / throws_per_cycle;
                skipped_throws = skipped_cycles * throws_per_cycle;
                cycle_found = true;
            }
        }
        if skipped_throws + shapes_thrown == total_throws {
            break;
        }
    }
    sim.height() as usize + skipped_cycles * cycle_height
}

fn calculate_solution(jet_pattern: &str) -> (usize, usize) {
    let moves = parse_moves(jet_pattern);
    (
        compute_rock_tower_height(&moves, 2022),
        compute_rock_tower_height(&moves, 1_000_000_000_000),
    )
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::JET_PATTERN));
}
