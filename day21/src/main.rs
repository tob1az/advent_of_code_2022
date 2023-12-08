mod data;

use std::collections::HashMap;

type Number = i64;
type Name = String;

enum Operation {
    Assign { value: Number },
    Add { operands: [Name; 2] },
    Subtract { operands: [Name; 2] },
    Multiply { operands: [Name; 2] },
    Divide { operands: [Name; 2] },
}

struct Solver {
    operations: HashMap<Name, Operation>,
}

impl Solver {
    fn solve(self) -> Number {
        let mut variables = self
            .operations
            .iter()
            .filter_map(|o| match o {
                (name, Operation::Assign { value }) => Some((name.clone(), *value)),
                _ => None,
            })
            .collect::<HashMap<Name, Number>>();
        loop {
            for (name, operation) in &self.operations {
                if !variables.contains_key(name) {
                    match operation {
                        Operation::Add { operands } => {
                            let a = variables.get(&operands[0]);
                            let b = variables.get(&operands[1]);
                            if a.is_some() && b.is_some() {
                                variables.insert(name.clone(), a.unwrap() + b.unwrap());
                            }
                        }
                        Operation::Subtract { operands } => {
                            let a = variables.get(&operands[0]);
                            let b = variables.get(&operands[1]);
                            if a.is_some() && b.is_some() {
                                variables.insert(name.clone(), a.unwrap() - b.unwrap());
                            }
                        }
                        Operation::Multiply { operands } => {
                            let a = variables.get(&operands[0]);
                            let b = variables.get(&operands[1]);
                            if a.is_some() && b.is_some() {
                                variables.insert(name.clone(), a.unwrap() * b.unwrap());
                            }
                        }
                        Operation::Divide { operands } => {
                            let a = variables.get(&operands[0]);
                            let b = variables.get(&operands[1]);
                            if a.is_some() && b.is_some() {
                                variables.insert(name.clone(), a.unwrap() / b.unwrap());
                            }
                        }
                        _ => (),
                    }
                } else if name == "root" {
                    return variables[name];
                }
            }
        }
    }
}

fn parse_operation(input: &str) -> HashMap<Name, Operation> {
    input
        .lines()
        .map(|l| {
            let (name, expr) = l.split_once(": ").unwrap();
            let name = name.to_owned();
            let operation = if let Ok(value) = expr.parse() {
                Operation::Assign { value }
            } else if let Some((a, b)) = expr.split_once(" + ") {
                Operation::Add {
                    operands: [a.to_owned(), b.to_owned()],
                }
            } else if let Some((a, b)) = expr.split_once(" - ") {
                Operation::Subtract {
                    operands: [a.to_owned(), b.to_owned()],
                }
            } else if let Some((a, b)) = expr.split_once(" * ") {
                Operation::Multiply {
                    operands: [a.to_owned(), b.to_owned()],
                }
            } else if let Some((a, b)) = expr.split_once(" / ") {
                Operation::Divide {
                    operands: [a.to_owned(), b.to_owned()],
                }
            } else {
                unreachable!()
            };
            (name, operation)
        })
        .collect()
}

fn calculate_solution(input: &str) -> Number {
    let operations = parse_operation(input);
    let solver = Solver { operations };
    solver.solve()
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
