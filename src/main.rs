extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

pub mod app;
pub mod food;
pub mod position;
pub mod snake;

use app::App;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{PressEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use position::Position;
use rand::Rng;
use snake::{Direction, Snake};

use crate::food::Food;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("snake", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let font =
        GlyphCache::new("assets/JBF.ttf", (), TextureSettings::new()).expect("Could not load font");

    let mut rng = rand::rng();

    let mut walls: Vec<Position> = vec![];
    for _ in 1..50 {
        walls.push(Position::new(
            rng.random_range(1..600) as f64,
            rng.random_range(1..600) as f64,
        ));
    }

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(Position::new(100.0, 100.0), Direction::East),
        food: Food {
            pos: Position::new(500.0, 500.0),
            count: 1,
        },
        walls,
        font,
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
