mod data;

type Number = isize;
type Snafu = String;
type SnafuRef = str;

const SNAFU_BASE: Number = 5;
const SNAFU_DIGITS: [char; SNAFU_BASE as usize] = ['=', '-', '0', '1', '2'];
const ZERO_OFFSET: Number = 2;

fn snafu_to_decimal<S: AsRef<SnafuRef>>(snafu: S) -> Number {
    let mut number = 0;
    for digit in snafu.as_ref().chars() {
        number *= SNAFU_BASE;
        number += SNAFU_DIGITS.iter().position(|c| *c == digit).unwrap() as Number - ZERO_OFFSET;
    }
    number
}

fn decimal_to_snafu(mut decimal: Number) -> Snafu {
    assert!(decimal >= 0);
    if decimal == 0 {
        return "0".to_owned();
    }
    let mut snafu = Vec::new();
    while decimal > 0 {
        let i = (decimal + ZERO_OFFSET) % SNAFU_BASE;
        snafu.push(SNAFU_DIGITS[i as usize]);
        // - or =
        if i < ZERO_OFFSET {
            decimal += SNAFU_BASE;
        }
        decimal /= SNAFU_BASE;
    }
    snafu.into_iter().rev().collect()
}

fn calculate_solution(input: &str) -> Snafu {
    decimal_to_snafu(input.lines().map(snafu_to_decimal).sum())
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snafu_to_decimal() {
        let test_data = data::TEST_SNAFU_DECIMAL
            .lines()
            .map(|l| {
                let (a, b) = l.trim().split_once(' ').unwrap();
                (a.trim().to_owned(), b.trim().parse::<Number>().unwrap())
            })
            .collect::<Vec<_>>();
        for (snafu, decimal) in test_data {
            assert_eq!(snafu_to_decimal(snafu), decimal);
        }
    }

    #[test]
    fn test_decimal_to_snafu() {
        let test_data = data::TEST_DECIMAL_SNAFU
            .lines()
            .map(|l| {
                let (a, b) = l.trim().split_once(' ').unwrap();
                (a.trim().parse::<Number>().unwrap(), b.trim().to_owned())
            })
            .collect::<Vec<_>>();
        for (decimal, snafu) in test_data {
            assert_eq!(decimal_to_snafu(decimal), snafu);
        }
    }
}
