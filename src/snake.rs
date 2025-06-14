use crate::position::Position;

pub enum SnakeStates {
    Alive,
    SelfEaten,
    Smashed,
}

#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Snake {
    pub head: Position,
    pub tail: Vec<Position>,
    pub len: u32,
    pub direction: Direction,
    pub state: SnakeStates,
}

impl Snake {
    pub fn new(pos: Position, dir: Direction) -> Snake {
        Snake {
            head: pos,
            tail: vec![],
            len: 1,
            direction: dir,
            state: SnakeStates::Alive,
        }
    }
    pub fn grow(&mut self) {
        self.len += 1;
        let last = self.tail.last();
        match last {
            Some(l) => self.tail.push(l.clone()),
            None => self.tail.push(self.head.clone()),
        }
    }
}
