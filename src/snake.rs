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
    pub fn next(&mut self) {
        let Position(x, y) = self.head;
        let mut prev = self.head.clone();

        match self.direction {
            Direction::North => self.head = Position(x, y - crate::STEP as f32),
            Direction::East => self.head = Position(x + crate::STEP as f32, y),
            Direction::South => self.head = Position(x, y + crate::STEP as f32),
            Direction::West => self.head = Position(x - crate::STEP as f32, y),
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
    pub fn smash(&mut self) {
        self.state = SnakeStates::Smashed;
    }

    pub fn change_direction(
        &mut self,
        opposite_direction: Direction,
        desired_direction: Direction,
    ) {
        if self.direction != opposite_direction {
            self.direction = desired_direction
        } else {
            self.state = SnakeStates::SelfEaten;
        }
    }
}
