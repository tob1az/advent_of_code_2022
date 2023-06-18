#[derive(Debug)]
pub struct Interval {
    from: i64,
    to: i64,
}

impl Interval {
    pub fn new(from: i64, to: i64) -> Self {
        assert!(from <= to);
        Self { from, to }
    }

    pub fn merge(&self, another: &Self) -> Option<Self> {
        if self.len() == 0 || another.len() == 0 {
            return None;
        }
        if self.from <= another.to && another.from <= self.to {
            Some(Self {
                from: self.from.min(another.from),
                to: self.to.max(another.to),
            })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        (self.to - self.from) as usize
    }

    pub fn from(&self) -> i64 {
        self.from
    }

    pub fn to(&self) -> i64 {
        self.to
    }
}
