mod data;

use std::{collections::HashMap, fmt::Display};

type Number = i64;

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Add => "+",
                Self::Subtract => "-",
                Self::Multiply => "*",
                Self::Divide => "/",
            }
        )
    }
}

impl Operator {
    fn apply(&self, a: Number, b: Number) -> Number {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
            Self::Multiply => a * b,
            Self::Divide => a / b,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Add => Self::Subtract,
            Self::Subtract => Self::Add,
            Self::Multiply => Self::Divide,
            Self::Divide => Self::Multiply,
        }
    }
}

struct Binary {
    operator: Operator,
    arguments: [Operation; 2],
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {})",
            self.arguments[0], self.operator, self.arguments[1]
        )
    }
}

enum Operation {
    Assign { value: Number },
    Combine(Box<Binary>),
    HumanAssign { value: Number },
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Assign { value } => write!(f, "{}", value),
            Self::HumanAssign { value } => write!(f, "H={}", value),
            Self::Combine(binary) => write!(f, "{}", binary),
        }
    }
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
        let (operator, right_name) = rest.split_once(' ').unwrap();
        let operator = match operator {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => unreachable!(),
        };
        let left_expression = other_operations.get(left_name).unwrap();
        let right_expression = other_operations.get(right_name).unwrap();
        Self::Combine(Box::new(Binary {
            operator,
            arguments: [
                Self::new(left_name, *left_expression, other_operations),
                Self::new(right_name, *right_expression, other_operations),
            ],
        }))
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
            Operation::Combine(binary) => binary.operator.apply(
                self.calculate(&binary.arguments[0]),
                self.calculate(&binary.arguments[1]),
            ),
        }
    }

    fn solve_human_value(&self) -> Number {
        // root is considered as equation "f(human) == constant"
        match &self.root {
            Operation::Combine(binary) => {
                let mut human_path = vec![];
                assert!(self.trace_human(&self.root, &mut human_path));
                assert!(!human_path.is_empty());
                let human_index = human_path[0];
                let constant_index = human_index ^ 1;
                let constant = self.calculate(&binary.arguments[constant_index]);
                self.undo_inhuman_operations(
                    &binary.arguments[human_index],
                    constant,
                    1,
                    &human_path,
                )
            }
            _ => panic!("root is not a binary operation!"),
        }
    }

    fn undo_inhuman_operations(
        &self,
        operation: &Operation,
        constant: Number,
        depth: usize,
        human_path: &[usize],
    ) -> Number {
        match operation {
            Operation::HumanAssign { .. } => constant,
            Operation::Combine(binary) => {
                let human_index = human_path[depth];
                let operand = self.calculate(&binary.arguments[human_index ^ 1]);
                // undo
                let constant = match binary.operator {
                    Operator::Subtract => {
                        if human_index == 0 {
                            // h - o = c  => h = o + c
                            operand + constant
                        } else {
                            // o - h = c  => h = o - c
                            operand - constant
                        }
                    }
                    Operator::Divide => {
                        if human_index == 0 {
                            // h / o = c  =>  h = c * o
                            constant * operand
                        } else {
                            // o / h = c  =>  h = c / o
                            constant / operand
                        }
                    }
                    _ => binary.operator.opposite().apply(constant, operand),
                };
                self.undo_inhuman_operations(
                    &binary.arguments[human_index],
                    constant,
                    depth + 1,
                    human_path,
                )
            }
            Operation::Assign { .. } => unreachable!(),
        }
    }

    fn trace_human(&self, operation: &Operation, path: &mut Vec<usize>) -> bool {
        match operation {
            Operation::HumanAssign { .. } => return true,
            Operation::Assign { .. } => return false,
            Operation::Combine(binary) => {
                path.push(0);
                if self.trace_human(&binary.arguments[0], path) {
                    true
                } else {
                    let last = path.len() - 1;
                    path[last] = 1;
                    if self.trace_human(&binary.arguments[1], path) {
                        true
                    } else {
                        path.pop();
                        false
                    }
                }
            }
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

fn calculate_solution(input: &str) -> (Number, Number) {
    let root = parse_operations(input);
    let solver = Solver { root };
    (solver.calculate_root(), solver.solve_human_value())
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reference_case_part_1() {
        let root = parse_operations(data::TEST_INPUT);
        let solver = Solver { root };
        assert_eq!(solver.calculate_root(), 152);
    }

    #[test]
    fn reference_case_part_2() {
        let root = parse_operations(data::TEST_INPUT);
        let solver = Solver { root };
        assert_eq!(solver.solve_human_value(), 301);
    }
}
