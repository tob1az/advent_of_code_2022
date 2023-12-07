mod data;

type Number = isize;

#[derive(Clone, Copy, Debug)]
struct OrderedNumber {
    index: Number,
    value: Number,
}

impl std::fmt::Display for OrderedNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]={}", self.index, self.value)
    }
}

struct EncryptedFile(Vec<OrderedNumber>);

impl std::fmt::Display for EncryptedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;
        for n in &self.0 {
            write!(f, "{} ", n)?;
        }
        write!(f, "]")
    }
}

fn parse_file(file: &str) -> EncryptedFile {
    EncryptedFile(
        file.lines()
            .enumerate()
            .map(|(n, line)| OrderedNumber {
                index: n as Number,
                value: line.parse::<Number>().unwrap(),
            })
            .collect::<Vec<_>>(),
    )
}

impl EncryptedFile {
    fn mix(mut self) -> PlaintextFile {
        println!("mixing {}", self);
        let len = self.0.len() as Number;
        for index in 0..len {
            let (position, number) = self
                .0
                .iter()
                .enumerate()
                .find(|n| n.1.index == index)
                .or_else(|| {
                    println!("Could not find {index}: {}", self);
                    None
                })
                .unwrap();
            let number = number.clone();
            let mut insert = (position as Number + number.value) % (len - 1);
            if insert <= 0 {
                insert += len - 1;
            }
            let insert = insert as usize;
            if insert > position {
                self.0.copy_within((position + 1)..=insert, position);
                self.0[insert] = number.clone();
            } else if insert < position {
                self.0.copy_within(insert..position, insert + 1);
                self.0[insert] = number.clone();
            }
            // println!("mix #{index} move {}@{}: {}", number.value, insert, self);
        }
        let data = self.0.into_iter().map(|n| n.value).collect::<Vec<_>>();
        PlaintextFile {
            zero_offset: data.iter().position(|n| *n == 0).unwrap(),
            data,
        }
    }
}

struct PlaintextFile {
    data: Vec<Number>,
    zero_offset: usize,
}

impl PlaintextFile {
    fn look_up(&self, index: usize) -> Number {
        let index = (index + self.zero_offset) % self.data.len();
        self.data[index]
    }
}

fn calculate_solution(file: &str) -> Number {
    let encrypted = parse_file(file);
    let plaintext = encrypted.mix();
    println!("plaintext: {:?}", plaintext.data);
    plaintext.look_up(1000) + plaintext.look_up(2000) + plaintext.look_up(3000)
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::FILE));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reference_case_part_1() {
        assert_eq!(calculate_solution(data::TEST_FILE), 3);
    }
}
