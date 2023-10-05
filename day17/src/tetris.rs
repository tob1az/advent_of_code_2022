use std::collections::HashMap;

// origin is at bottom left corner
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Coordinates {
    x: i64,
    y: i64,
}

#[derive(Clone)]
pub struct Move {
    pub dx: i64,
    pub dy: i64,
}

#[derive(Clone, Debug)]
struct Shape {
    blocks: Vec<Coordinates>,
}

impl Coordinates {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Shape {
    fn new(blocks: &[Coordinates]) -> Self {
        assert!(!blocks.is_empty());
        Self {
            blocks: blocks.into(),
        }
    }
    fn replicate_at(&self, origin: &Coordinates) -> Self {
        let mut replica = self.clone();
        for block in replica.blocks.iter_mut() {
            block.x += origin.x;
            block.y += origin.y;
        }
        replica
    }
    fn can_move(
        &self,
        shape_move: &Move,
        chamber_width: i64,
        other_blocks: &[Coordinates],
    ) -> bool {
        for block in &self.blocks {
            let x = block.x + shape_move.dx;
            let y = block.y + shape_move.dy;
            if x < 0 || x >= chamber_width || y < 0 || other_blocks.contains(&Coordinates { x, y })
            {
                return false;
            }
        }
        true
    }
    fn make_move(&mut self, shape_move: &Move) {
        for block in self.blocks.iter_mut() {
            block.x += shape_move.dx;
            block.y += shape_move.dy;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct ThrowParameters {
    relative_heights: Vec<i64>,
    shape_index: usize,
    shift_index: usize,
}

#[derive(Debug, Clone)]
pub struct ThrowResult {
    pub height: i64,
    pub throw_count: usize,
}

#[derive(Debug)]
pub struct Simulation {
    width: i64,
    initial_x: i64,
    chamber: Vec<Coordinates>,
    shape_sequence: Vec<Shape>,
    shape_index: usize,
    move_sequence: Vec<i64>,
    move_index: usize,
    height: i64,
    shapes_thrown: usize,
    throws: HashMap<ThrowParameters, ThrowResult>,
}

impl Simulation {
    pub fn new(width: i64, initial_x: i64, move_sequence: &[i64]) -> Self {
        assert!(width > 0 && initial_x > 0 && initial_x < width);
        Self {
            width,
            initial_x,
            chamber: Vec::with_capacity(10_000),
            shape_sequence: build_shape_sequence(),
            shape_index: 0,
            move_sequence: move_sequence.to_owned(),
            move_index: 0,
            height: 0, // floor
            shapes_thrown: 0,
            throws: HashMap::new(),
        }
    }

    pub fn throw_next_shape(&mut self) {
        let mut shape = self.pick_next_shape();
        let down = Move { dx: 0, dy: -1 };
        loop {
            let shape_move = self.pick_next_move();
            if shape.can_move(&shape_move, self.width, &self.chamber) {
                shape.make_move(&shape_move)
            }
            // landed
            if !shape.can_move(&down, self.width, &self.chamber) {
                break;
            }
            shape.make_move(&down);
        }
        self.chamber.append(&mut shape.blocks);
        self.height = self.chamber.iter().max_by_key(|c| c.y).unwrap().y + 1;
        self.shapes_thrown += 1;
    }
    pub fn height(&self) -> i64 {
        self.height
    }
    fn pick_next_shape(&mut self) -> Shape {
        let shape = self.shape_sequence[self.shape_index].replicate_at(&Coordinates {
            x: self.initial_x,
            y: self.height + 3,
        });
        self.shape_index = (self.shape_index + 1) % self.shape_sequence.len();
        shape
    }
    fn pick_next_move(&mut self) -> Move {
        let dx = self.move_sequence[self.move_index];
        self.move_index = (self.move_index + 1) % self.move_sequence.len();
        Move { dx, dy: 0 }
    }
    pub fn find_throw_cycle(&mut self) -> Option<ThrowResult> {
        let mut heights = (0..self.width)
            .map(|x| {
                self.chamber
                    .iter()
                    .filter(|c| c.x == x)
                    .map(|c| c.y + 1)
                    .max()
                    .unwrap_or(0)
            })
            .collect::<Vec<_>>();
        let min_y = heights.iter().min().copied().unwrap();
        for h in heights.iter_mut() {
            *h -= min_y;
        }
        let key = ThrowParameters {
            relative_heights: heights,
            shape_index: self.shape_index,
            shift_index: self.move_index,
        };
        let result = self.throws.get(&key).cloned();
        if result.is_none() {
            self.throws.insert(
                key,
                ThrowResult {
                    height: self.height,
                    throw_count: self.shapes_thrown,
                },
            );
        }
        result
    }
}

fn build_shape_sequence() -> Vec<Shape> {
    vec![
        Shape::new(&vec![
            Coordinates::new(0, 0),
            Coordinates::new(1, 0),
            Coordinates::new(2, 0),
            Coordinates::new(3, 0),
        ]),
        Shape::new(&vec![
            Coordinates::new(1, 0),
            Coordinates::new(0, 1),
            Coordinates::new(1, 1),
            Coordinates::new(1, 2),
            Coordinates::new(2, 1),
        ]),
        Shape::new(&vec![
            Coordinates::new(0, 0),
            Coordinates::new(1, 0),
            Coordinates::new(2, 0),
            Coordinates::new(2, 1),
            Coordinates::new(2, 2),
        ]),
        Shape::new(&vec![
            Coordinates::new(0, 0),
            Coordinates::new(0, 1),
            Coordinates::new(0, 2),
            Coordinates::new(0, 3),
        ]),
        Shape::new(&vec![
            Coordinates::new(0, 0),
            Coordinates::new(0, 1),
            Coordinates::new(1, 0),
            Coordinates::new(1, 1),
        ]),
    ]
}
