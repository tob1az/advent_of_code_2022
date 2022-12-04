mod data;

fn score_round(round: &(char, char)) -> usize {
    const ROCK: usize = 1;
    const PAPER: usize = 2;
    const SCISSORS: usize = 3;
    const LOSS: usize = 0;
    const DRAW: usize = 3;
    const WIN: usize = 6;

    match round {
        ('A', 'X') => ROCK + DRAW,
        ('A', 'Y') => PAPER + WIN,
        ('A', 'Z') => SCISSORS + LOSS,
        ('B', 'X') => ROCK + LOSS,
        ('B', 'Y') => PAPER + DRAW,
        ('B', 'Z') => SCISSORS + WIN,
        ('C', 'X') => ROCK + WIN,
        ('C', 'Y') => PAPER + LOSS,
        ('C', 'Z') => SCISSORS + DRAW,
        _ => panic!("wrong shapes!"),
    }
}

fn calculate_solution(strategy: &[(char, char)]) -> usize {
    strategy.iter().map(score_round).sum()
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::STRATEGY));
}
