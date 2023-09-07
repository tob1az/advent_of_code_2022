// origin is at bottom left corner
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Clone)]
pub struct Move {
    pub dx: i32,
    pub dy: i32,
}

#[derive(Clone, Debug)]
struct Shape {
    blocks: Vec<Coordinates>,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
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
    fn replicate(&self, origin: &Coordinates) -> Self {
        let mut replica = self.clone();
        for block in replica.blocks.iter_mut() {
            block.x += origin.x;
            block.y += origin.y;
        }
        replica
    }
    fn can_move(&self, shape_move: &Move, chamber_width: i32, other_blocks: &[Coordinates]) -> bool {
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

#[derive(Debug)]
pub struct Simulation {
    width: i32,
    initial_x: i32,
    chamber: Vec<Coordinates>,
    shape_sequence: Vec<Shape>,
    shape_index: usize,
    move_sequence: Vec<i32>,
    move_index: usize,
    height: i32, 
}

impl Simulation {
    pub fn new(width: i32, initial_x: i32, move_sequence: &[i32]) -> Self {
        assert!(
            width > 0
                && initial_x > 0
                && initial_x < width
        );
        Self {
            width,
            initial_x,
            chamber: Vec::with_capacity(10_000),
            shape_sequence: build_shape_sequence(),
            shape_index: 0,
            move_sequence: move_sequence.to_owned(),
            move_index: 0,
            height: 0, // floor
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
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    fn pick_next_shape(&mut self) -> Shape {
        let shape = self.shape_sequence[self.shape_index].replicate(&Coordinates { x: self.initial_x, y: self.height + 3 });
        self.shape_index = (self.shape_index + 1) % self.shape_sequence.len();
        shape
    }
    fn pick_next_move(&mut self) -> Move {
        let dx = self.move_sequence[self.move_index];
        self.move_index = (self.move_index + 1) % self.move_sequence.len();
        Move { dx, dy: 0 }
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
