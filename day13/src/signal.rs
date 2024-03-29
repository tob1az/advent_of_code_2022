use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum Packet {
    Digit(u32),
    Nested(Vec<Packet>),
}

impl Packet {
    pub fn parse(string: &str) -> Option<Self> {
        let mut nested_packets = Vec::new();
        let mut number = String::new();
        for c in string.chars() {
            match c {
                '[' => nested_packets.push(Vec::default()),
                '0'..='9' => {
                    number.push(c);
                }
                ',' | ']' => {
                    if !number.is_empty() {
                        nested_packets
                            .last_mut()?
                            .push(Packet::Digit(number.parse().ok()?));
                        number.clear()
                    }
                    if c == ']' {
                        match nested_packets.len() {
                            2.. => {
                                let packet = nested_packets.pop()?;
                                nested_packets.last_mut()?.push(Packet::Nested(packet));
                            }
                            1 => return Some(Packet::Nested(nested_packets.pop()?)),
                            _ => return None,
                        }
                    }
                }
                _ => return None,
            }
        }
        None
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Packet::*;
        match (self, other) {
            (Digit(l), Digit(r)) => l.partial_cmp(r),
            (Nested(l), Nested(r)) => l.partial_cmp(r),
            (Digit(_), Nested(r)) => vec![self.clone()].partial_cmp(r),
            (Nested(l), Digit(_)) => l.partial_cmp(&vec![other.clone()]),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Digit(l), Digit(r)) => l.cmp(r),
            (Nested(l), Nested(r)) => l.cmp(r),
            (Digit(_), Nested(r)) => vec![self.clone()].cmp(r),
            (Nested(l), Digit(_)) => l.cmp(&vec![other.clone()]),
        }
    }
}
