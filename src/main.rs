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
        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let snake = rectangle::square(0.0, 0.0, 50.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(FOREGROUND, snake, c.transform.trans(-40.0, 10.0), gl);
        });
    }
    fn update(&mut self, _args: &UpdateArgs) {
        println!("update");
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    // up
                }
                Key::Down => {
                    // down
                }
                Key::Left => {
                    // Left
                }
                Key::Right => {
                    // Right
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("snake", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(Position(10.0, 10.0), Direction::East),
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
