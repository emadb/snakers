#[derive(Clone)]
pub struct Position(pub f32, pub f32);

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position(x, y)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1)
    }
}
