use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::{Button, Key, RenderArgs, UpdateArgs};

use crate::food::Food;
use crate::position::Position;
use crate::snake::{Direction, Snake, SnakeStates};

const STEP: i32 = 10;

pub struct App {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub food: Food,
    pub font: GlyphCache<'static>,
    pub walls: Vec<Position>,
}
impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const SNAKE: [f32; 4] = [0.0, 0.4, 1.0, 1.0];
        const FOOD: [f32; 4] = [0.9, 0.1, 0.1, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const WALL: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

        match self.snake.state {
            SnakeStates::Alive => {
                let Position(x, y) = self.snake.head;
                let snake = rectangle::square(x, y, 10.0);
                let food = rectangle::square(self.food.pos.0, self.food.pos.1, 10.0);

                self.gl.draw(args.viewport(), |c, gl| {
                    clear(BACKGROUND, gl);

                    for w in self.walls.clone() {
                        rectangle(
                            WALL,
                            rectangle::square(w.0, w.1, 10.0),
                            c.transform.trans(0.0, 0.0),
                            gl,
                        );
                    }

                    text(
                        WHITE,
                        32,
                        self.snake.len.to_string().as_str(),
                        &mut self.font,
                        c.transform.trans(10.0, 50.0),
                        gl,
                    )
                    .unwrap();
                    rectangle(FOOD, food, c.transform.trans(0.0, 0.0), gl);
                    rectangle(SNAKE, snake, c.transform.trans(0.0, 0.0), gl);
                    for sn in &self.snake.tail {
                        let Position(xs, ys) = sn;
                        let t = rectangle::square(*xs, *ys, 10.0);
                        rectangle(SNAKE, t, c.transform.trans(0.0, 0.0), gl);
                    }
                });
            }
            SnakeStates::SelfEaten => self.gl.draw(args.viewport(), |c, gl| {
                clear(BACKGROUND, gl);

                text(
                    WHITE,
                    32,
                    "GAME OVER :-(",
                    &mut self.font,
                    c.transform.trans(200.0, 250.0),
                    gl,
                )
                .unwrap();
            }),
            SnakeStates::Smashed => {}
        }
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        let Position(x, y) = self.snake.head;
        let mut prev = self.snake.head.clone();

        match self.snake.direction {
            Direction::North => self.snake.head = Position(x, y - STEP as f64),
            Direction::East => self.snake.head = Position(x + STEP as f64, y),
            Direction::South => self.snake.head = Position(x, y + STEP as f64),
            Direction::West => self.snake.head = Position(x - STEP as f64, y),
        }

        for sn in &mut self.snake.tail {
            let temp = sn.clone();
            *sn = prev;
            prev = temp;
        }

        if self.snake.tail.contains(&self.snake.head) {
            self.snake.state = SnakeStates::SelfEaten;
        }

        if self.snake.head == self.food.pos {
            self.snake.grow();
            self.food.next_pos();
        }
    }

    pub fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    if self.snake.direction != Direction::South {
                        self.snake.direction = Direction::North
                    } else {
                        self.snake.state = SnakeStates::SelfEaten;
                    }
                }
                Key::Down => {
                    if self.snake.direction != Direction::North {
                        self.snake.direction = Direction::South
                    } else {
                        self.snake.state = SnakeStates::SelfEaten;
                    }
                }
                Key::Left => {
                    if self.snake.direction != Direction::East {
                        self.snake.direction = Direction::West
                    } else {
                        self.snake.state = SnakeStates::SelfEaten;
                    }
                }
                Key::Right => {
                    if self.snake.direction != Direction::West {
                        self.snake.direction = Direction::East
                    } else {
                        self.snake.state = SnakeStates::SelfEaten;
                    }
                }
                _ => {}
            }
        }
    }
}
