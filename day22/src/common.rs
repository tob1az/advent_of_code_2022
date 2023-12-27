pub type Number = isize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MovingPoint {
    pub direction: Direction,
    pub column: Number,
    pub row: Number,
}

