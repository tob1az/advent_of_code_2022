mod data;

type WorryLevel = usize;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<WorryLevel>,
    operation: Operation,
    test_divisor: WorryLevel,
    true_monkey_number: usize,
    false_monkey_number: usize,
    inspection_count: usize,
}

fn last_number(line: &str) -> usize {
    line.rsplit_once(' ').unwrap().1.parse::<usize>().unwrap()
}

impl Monkey {
    fn new(monkey_note: &str) -> Self {
        let lines = monkey_note.lines().skip(1).collect::<Vec<_>>();
        let starting_items = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|i| i.parse::<WorryLevel>().unwrap())
            .collect::<Vec<_>>();
        let operation = match lines[1]
            .split_once("= old ")
            .unwrap()
            .1
            .split_once(' ')
            .unwrap()
        {
            ("+", level) => Operation::Add(level.parse::<WorryLevel>().unwrap()),
            ("*", "old") => Operation::Square,
            ("*", level) => Operation::Multiply(level.parse::<WorryLevel>().unwrap()),
            _ => panic!("unsupported operation"),
        };
        let test_divisor = last_number(lines[2]);
        let true_monkey_number = last_number(lines[3]);
        let false_monkey_number = last_number(lines[4]);
        Self {
            items: starting_items,
            operation,
            test_divisor,
            true_monkey_number,
            false_monkey_number,
            inspection_count: 0,
        }
    }

    fn inspect_item(&self, item: WorryLevel) -> WorryLevel {
        match &self.operation {
            Operation::Add(x) => item + x,
            Operation::Multiply(x) => item * x,
            Operation::Square => item * item,
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(WorryLevel),
    Multiply(WorryLevel),
    Square,
}

fn level_of_monkey_business(monkeys: Vec<Monkey>, relief_divisor: usize, rounds: usize) -> usize {
    let mut monkeys = monkeys;
    // limit the worry level by applying the Chinese Remainder Theorem (all divisors are primes)
    let gcd = monkeys
        .iter()
        .map(|m| m.test_divisor)
        .product::<WorryLevel>();
    let monkey_count = monkeys.len();
    for round in 1..=rounds {
        println!("Round {round}");
        for i in 0..monkey_count {
            let monkey = monkeys[i].clone();
            let inspection_count = monkey.items.len();
            monkey
                .items
                .iter()
                .map(|i| monkey.inspect_item(*i) / relief_divisor % gcd)
                .for_each(|i| {
                    let recipient_number = if i % monkey.test_divisor == 0 {
                        monkey.true_monkey_number
                    } else {
                        monkey.false_monkey_number
                    };
                    monkeys[recipient_number].items.push(i);
                });
            monkeys[i].items.clear();
            monkeys[i].inspection_count += inspection_count;
        }
    }
    monkeys.sort_by_key(|m| m.inspection_count);
    monkeys
        .iter()
        .rev()
        .map(|m| m.inspection_count)
        .take(2)
        .product()
}

fn calculate_solution(notes: &str) -> (usize, usize) {
    let monkeys = notes.split("\n\n").map(Monkey::new).collect::<Vec<_>>();
    (
        level_of_monkey_business(monkeys.clone(), 3, 20),
        level_of_monkey_business(monkeys, 1, 10000),
    )
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::MONKEY_NOTES));
}
