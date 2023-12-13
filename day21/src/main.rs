mod data;

use std::collections::HashMap;

type Number = i64;

enum Operand {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operand {
    fn apply(&self, a: Number, b: Number) -> Number {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
            Self::Multiply => a * b,
            Self::Divide => a / b,
        }
    }
}

enum Operation {
    Assign { value: Number },
    Binary(Box<(Operation, Operand, Operation)>),
    HumanAssign { value: Number },
}

impl Operation {
    fn new(name: &str, expression: &str, other_operations: &HashMap<&str, &str>) -> Self {
        if let Ok(value) = expression.parse() {
            return if name == "humn" {
                Self::HumanAssign { value }
            } else {
                Self::Assign { value }
            };
        }
        let (left_name, rest) = expression.split_once(' ').unwrap();
        let (operand, right_name) = rest.split_once(' ').unwrap();
        let operand = match operand {
            "+" => Operand::Add,
            "-" => Operand::Subtract,
            "*" => Operand::Multiply,
            "/" => Operand::Divide,
            _ => unreachable!(),
        };
        let left_expression = other_operations.get(left_name).unwrap();
        let right_expression = other_operations.get(right_name).unwrap();
        Self::Binary(Box::new((
            Self::new(left_name, *left_expression, other_operations),
            operand,
            Self::new(right_name, *right_expression, other_operations),
        )))
    }
}

struct Solver {
    root: Operation,
}

impl Solver {
    fn calculate_root(&self) -> Number {
        self.calculate(&self.root)
    }

    fn calculate(&self, operation: &Operation) -> Number {
        match operation {
            Operation::Assign { value } => *value,
            Operation::HumanAssign { value } => *value,
            Operation::Binary(binary) => binary
                .1
                .apply(self.calculate(&binary.0), self.calculate(&binary.2)),
        }
    }
}

fn parse_operations(input: &str) -> Operation {
    let expressions = input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .collect::<HashMap<_, _>>();
    let root = expressions.get("root").unwrap();
    Operation::new("root", *root, &expressions)
}

fn calculate_solution(input: &str) -> Number {
    let root = parse_operations(input);
    let solver = Solver { root };
    solver.calculate_root()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reference_case_part_1() {
        assert_eq!(calculate_solution(data::TEST_INPUT), 152);
    }
}
