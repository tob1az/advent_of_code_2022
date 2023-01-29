mod data;

type Value = i32;
type Cycle = Value;

enum Instruction {
    Noop,
    AddX(Value),
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        let parts = instruction.split(' ').collect::<Vec<_>>();
        if parts[0] == "noop" {
            return Self::Noop;
        }
        if parts.len() == 2 && parts[0] == "addx" {
            if let Ok(value) = parts[1].parse::<Value>() {
                return Self::AddX(value);
            }
        }
        panic!("Illegal instruction: {instruction}");
    }

    fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

struct Computer {
    x: Value,
}

impl Computer {
    fn new() -> Self {
        Self { x: 1 }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::AddX(value) => self.x += value,
            _ => (),
        }
    }

    fn compute_x_for_each_cycle(&mut self, program: &str) -> Vec<(Cycle, Value)> {
        program
            .lines()
            .map(|l| {
                let instruction = Instruction::new(l);
                let previous_x = self.x;
                self.execute(&instruction);
                std::iter::repeat(previous_x).take(instruction.cycles())
            })
            .flatten()
            .enumerate()
            .map(|(i, x)| ((i + 1) as Value, x))
            .collect::<Vec<_>>()
    }
}

fn render_sprite_pixel_at(sprite_position: Value, position: Value) -> char {
    if position == sprite_position
        || position == sprite_position - 1
        || position == sprite_position + 1
    {
        '#'
    } else {
        '.'
    }
}

fn compute_total_rightmost_signal_strength(x_positions: &[(Cycle, Value)]) -> Value {
    x_positions
        .iter()
        .map(|(i, x)| match i {
            20 | 60 | 100 | 140 | 180 | 220 => {
                let s = (*i as Value) * x;
                println!("x = {x} i = {}, score = {s}", i + 1);
                s
            }
            _ => 0,
        })
        .sum::<Value>()
}

fn render_frame(x_positions: &[(Cycle, Value)]) -> String {
    x_positions
        .iter()
        .map(|(i, x)| {
            const ROW_LENGTH: Value = 40;
            let pixel_position = (i - 1) % ROW_LENGTH;
            let pixel = render_sprite_pixel_at(*x, pixel_position);
            if pixel_position == ROW_LENGTH - 1 {
                vec![pixel, '\n']
            } else {
                vec![pixel]
            }
        })
        .flatten()
        .collect()
}

fn calculate_solution(program: &str) -> (Value, String) {
    let x_positions = Computer::new().compute_x_for_each_cycle(program);
    let total_signal_strength = compute_total_rightmost_signal_strength(&x_positions);
    let frame = render_frame(&x_positions);
    (total_signal_strength, frame)
}

fn main() {
    let (total, frame) = calculate_solution(data::PROGRAM);
    println!("Solution {total}\n{frame}");
}
