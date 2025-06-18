use macroquad::prelude::*;
pub mod app;
pub mod food;
pub mod lcg;
pub mod position;
pub mod snake;

use app::App;

pub const STEP: i32 = 10;
pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 600.0;
pub const SNAKE_UPDATE_INTERVAL: f64 = 0.05;

#[macroquad::main("snakers")]
async fn main() {
    let mut app = App::new();
    let mut last_update_time = get_time();
    loop {
        clear_background(BLACK);
        app.handle_keys();
        if get_time() - last_update_time > SNAKE_UPDATE_INTERVAL {
            last_update_time = get_time();
            app.update();
        }
        app.render();
        next_frame().await;
    }
}
