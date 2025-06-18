use crate::lcg::*;
use crate::position::Position;

pub struct Food {
    pub pos: Position,
    pub count: u32,
}

impl Food {
    pub fn create_new(&mut self) {
        let mut rng = Lcg::new();
        let rx = rng.gen_range(0, 60) as i32;
        let ry = rng.gen_range(0, 60) as i32;
        self.count += 1;
        let new_x = (rx * crate::STEP) as f32;
        let new_y = (ry * crate::STEP) as f32;
        self.pos = Position::new(new_x, new_y);
    }
}
