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

const STEP: i32 = 10;

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
    pub fn next(&mut self) {
        let Position(x, y) = self.head;
        let mut prev = self.head.clone();

        match self.direction {
            Direction::North => self.head = Position(x, y - STEP as f64),
            Direction::East => self.head = Position(x + STEP as f64, y),
            Direction::South => self.head = Position(x, y + STEP as f64),
            Direction::West => self.head = Position(x - STEP as f64, y),
        }

        for sn in &mut self.tail {
            let temp = sn.clone();
            *sn = prev;
            prev = temp;
        }

        if self.tail.contains(&self.head) {
            self.state = SnakeStates::SelfEaten;
        }
    }
}
