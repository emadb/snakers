extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use rand::Rng;

use piston::window::WindowSettings;
use std::process;

struct Position(f64, f64);

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1)
    }
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Snake {
    head: Position,
    len: u32,
    direction: Direction,
}

struct Food {
    pos: Position,
    count: u32,
}

impl Food {
    fn next_pos(self: &mut Self) {
        let mut rng = rand::rng();
        self.count += 1;
        let new_x = (rng.random_range(0..60) * 10) as f64;
        let new_y = (rng.random_range(0..60) * 10) as f64;
        self.pos = Position(new_x, new_y);
    }
}

impl Snake {
    fn new(pos: Position, dir: Direction) -> Snake {
        Snake {
            head: pos,
            len: 2,
            direction: dir,
        }
    }
}

pub struct App {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const SNAKE: [f32; 4] = [0.0, 0.4, 1.0, 1.0];
        const FOOD: [f32; 4] = [0.9, 0.1, 0.1, 1.0];

        let Position(x, y) = self.snake.head;
        let snake = rectangle::square(x, y, 10.0);
        let food = rectangle::square(self.food.pos.0, self.food.pos.1, 10.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(SNAKE, snake, c.transform.trans(0.0, 0.0), gl);

            rectangle(FOOD, food, c.transform.trans(0.0, 0.0), gl);
        });
    }
    fn update(&mut self, _args: &UpdateArgs) {
        let Position(x, y) = self.snake.head;

        let step = 10.0;

        match self.snake.direction {
            Direction::North => self.snake.head = Position(x, y - step),
            Direction::East => self.snake.head = Position(x + step, y),
            Direction::South => self.snake.head = Position(x, y + step),
            Direction::West => self.snake.head = Position(x - step, y),
        }

        println!(
            "SNAKE: ?{} ?{}   FOOD: ?{} ?{}",
            self.snake.head.0, self.snake.head.1, self.food.pos.0, self.food.pos.1
        );

        if self.snake.head == self.food.pos {
            self.food.next_pos()
        }
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    if self.snake.direction != Direction::South {
                        self.snake.direction = Direction::North
                    }
                }
                Key::Down => {
                    if self.snake.direction != Direction::North {
                        self.snake.direction = Direction::South
                    }
                }
                Key::Left => {
                    if self.snake.direction != Direction::East {
                        self.snake.direction = Direction::West
                    }
                }
                Key::Right => {
                    if self.snake.direction != Direction::West {
                        self.snake.direction = Direction::East
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("snake", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(Position(100.0, 100.0), Direction::East),
        food: Food {
            pos: Position(500.0, 500.0),
            count: 1,
        },
    };

    let mut events = Events::new(EventSettings::new().max_fps(20).ups(20));

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.press_args() {
            app.press(&b);
        }

        // if let Some(b) = e.release_args() {
        //     app.release(&b);
        // }
    }
}
