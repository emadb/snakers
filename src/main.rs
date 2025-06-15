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
use opengl_graphics::OpenGL;
use piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{PressEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

pub const STEP: i32 = 10;
pub const WIDTH: f64 = 600.0;
pub const HEIGHT: f64 = 600.0;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("snake", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl);

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
