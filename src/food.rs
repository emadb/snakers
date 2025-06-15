use crate::position::Position;
use rand::Rng;

pub struct Food {
    pub pos: Position,
    pub count: u32,
}

impl Food {
    pub fn create_new(&mut self) {
        let mut rng = rand::rng();
        self.count += 1;
        let new_x = (rng.random_range(0..60) * crate::STEP) as f32;
        let new_y = (rng.random_range(0..60) * crate::STEP) as f32;
        self.pos = Position::new(new_x, new_y);
    }
}
