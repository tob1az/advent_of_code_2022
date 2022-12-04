mod data;

type Score = usize;

const ROCK: Score = 1;
const PAPER: Score = 2;
const SCISSORS: Score = 3;
const LOSS: Score = 0;
const DRAW: Score = 3;
const WIN: Score = 6;

fn score_round(round: &(char, char)) -> Score {
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

fn play_round(round: &(char, char)) -> Score {
    match round {
        ('A', 'X') => SCISSORS + LOSS,
        ('A', 'Y') => ROCK + DRAW,
        ('A', 'Z') => PAPER + WIN,

        ('B', 'X') => ROCK + LOSS,
        ('B', 'Y') => PAPER + DRAW,
        ('B', 'Z') => SCISSORS + WIN,

        ('C', 'X') => PAPER + LOSS,
        ('C', 'Y') => SCISSORS + DRAW,
        ('C', 'Z') => ROCK + WIN,
        _ => panic!("wrong shapes!"),
    }
}

fn calculate_solution(strategy: &[(char, char)]) -> (Score, Score) {
    let presumed_score = strategy.iter().map(score_round).sum();
    let real_score = strategy.iter().map(play_round).sum();
    (presumed_score, real_score)
}

fn main() {
    println!("Solution {:?}", calculate_solution(&data::STRATEGY));
}
