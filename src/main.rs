extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};

use piston::window::WindowSettings;
use std::process;

struct Position(f64, f64);

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
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.4, 1.0, 1.0];

        let Position(x, y) = self.snake.head;
        let snake = rectangle::square(x, y, 10.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(FOREGROUND, snake, c.transform.trans(0.0, 0.0), gl);
        });
    }
    fn update(&mut self, _args: &UpdateArgs) {
        let Position(x, y) = self.snake.head;

        let step = 1.0;

        match self.snake.direction {
            Direction::North => self.snake.head = Position(x, y - step),
            Direction::East => self.snake.head = Position(x + step, y),
            Direction::South => self.snake.head = Position(x, y + step),
            Direction::West => self.snake.head = Position(x - step, y),
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
    };

    let mut events = Events::new(EventSettings::new());
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
