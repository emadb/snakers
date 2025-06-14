#[derive(Clone)]
pub struct Position(pub f64, pub f64);

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position(x, y)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1)
    }
}
