use crate::position::Position;
use rand::Rng;

const STEP: i32 = 10;

pub struct Food {
    pub pos: Position,
    pub count: u32,
}

impl Food {
    pub fn next_pos(&mut self) {
        let mut rng = rand::rng();
        self.count += 1;
        let new_x = (rng.random_range(0..60) * STEP) as f64;
        let new_y = (rng.random_range(0..60) * STEP) as f64;
        self.pos = Position::new(new_x, new_y);
    }
}
