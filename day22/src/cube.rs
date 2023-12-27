use crate::common::{Direction, MovingPoint, Number};

pub struct Configuration {
    pub side_length: Number,
    pub edges: Vec<Edge>,
}

impl Configuration {
    pub fn transform_point<F>(&self, point: &MovingPoint, is_correct_point: F) -> MovingPoint
    where
        F: Fn(&MovingPoint) -> bool,
    {
        let edge = self
            .edges
            .iter()
            .find(|e| e.contains(point, self.side_length))
            .or_else(|| {
                panic!("Could not find edge for point {point:?}");
            })
            .unwrap();
        dbg!(&edge);
        let mut new_point = (*point).clone();
        edge.trans.apply(&mut new_point, self.side_length);
        println!("{:?} -> {:?}", point, &new_point);
        // Only 3 angles (90, 180, 270) are supported around nearest angle to the top left,
        // but IRL they can be negative and have different centers of rotation.
        // To compensate this inaccuracy, a correction step was introduced
        if !is_correct_point(&new_point) {
            edge.trans.correct(&mut new_point, self.side_length);
            println!("corrected: {:?}", &new_point);
            assert!(is_correct_point(&new_point));
        }
        new_point
    }
}

// length in cube side lengths
type CubeNumber = Number;

#[derive(Debug)]
pub struct Edge {
    pub direction: Direction,
    pub level: CubeNumber,
    pub offset: CubeNumber,
    pub trans: Transformation,
}

impl Edge {
    pub const fn new(
        direction: Direction,
        level: CubeNumber,
        offset: CubeNumber,
        new_direction: Direction,
        rotation: Rotation,
        delta_x: CubeNumber,
        delta_y: CubeNumber,
    ) -> Self {
        Self {
            direction,
            level,
            offset,
            trans: Transformation {
                direction: new_direction,
                rotation,
                delta_x,
                delta_y,
            },
        }
    }

    fn contains(&self, point: &MovingPoint, side_length: Number) -> bool {
        if point.direction != self.direction {
            return false;
        }
        let from = side_length * self.offset;
        let to = from + side_length - 1;
        let level = side_length * self.level;
        match point.direction {
            Direction::Up => level == point.row && point.column >= from && point.column <= to,
            Direction::Down => level == point.row + 1 && point.column >= from && point.column <= to,
            Direction::Right => level == point.column + 1 && point.row >= from && point.row <= to,
            Direction::Left => level == point.column && point.row >= from && point.row <= to,
        }
    }
}

#[derive(Debug)]
pub struct Transformation {
    pub direction: Direction,
    pub rotation: Rotation,
    pub delta_x: CubeNumber,
    pub delta_y: CubeNumber,
}

impl Transformation {
    fn apply(&self, point: &mut MovingPoint, side_length: Number) {
        self.rotation.rotate(point, side_length);
        point.column += side_length * self.delta_x;
        point.row += side_length * self.delta_y;
        point.direction = self.direction;
    }
    fn correct(&self, point: &mut MovingPoint, side_length: Number) {
        self.rotation.correct(point, side_length);
    }
}

#[derive(Debug)]
pub enum Rotation {
    None,
    By90,
    By180,
    By270,
}

impl Rotation {
    fn rotate(&self, point: &mut MovingPoint, side_length: Number) {
        if matches!(self, Rotation::None) {
            return;
        }
        // find rotation center by truncation of coordinates to nearest side edge
        // consider + 1 if point goes to another cube side during rotation
        match point.direction {
            Direction::Left | Direction::Right => {
                let center_row = point.row / side_length * side_length;
                let radius = point.row - center_row;
                let correction = if matches!(point.direction, Direction::Left) {
                    1
                } else {
                    0
                };
                match self {
                    Rotation::None => (),
                    Rotation::By90 => {
                        point.column -= radius + correction;
                        point.row = center_row;
                    }
                    Rotation::By180 => {
                        point.row = center_row - radius - 1;
                    }
                    Rotation::By270 => {
                        point.column += radius + 1 - correction;
                        point.row = center_row;
                    }
                }
            }
            Direction::Up | Direction::Down => {
                let center_column = point.column / side_length * side_length;
                let radius = point.column - center_column;
                let correction = if matches!(point.direction, Direction::Down) {
                    1
                } else {
                    0
                };
                match self {
                    Rotation::None => (),
                    Rotation::By90 => {
                        point.row += radius + correction;
                        point.column = center_column;
                    }
                    Rotation::By180 => {
                        point.column = center_column - radius - 1;
                    }
                    Rotation::By270 => {
                        point.row -= radius - correction;
                        point.column = center_column;
                    }
                }
            }
        };
    }

    fn correct(&self, point: &mut MovingPoint, side_length: Number) {
        match point.direction {
            Direction::Down | Direction::Up => {
                if point.row % side_length == 0 {
                    point.row -= 1;
                } else {
                    point.row += 1;
                }
            }
            Direction::Left | Direction::Right => {
                if point.column % side_length == 0 {
                    point.column -= 1;
                } else {
                    point.column += 1;
                }
            }
        }
    }
}
