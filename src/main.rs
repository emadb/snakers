use macroquad::{color, prelude::*};
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
    loop {
        draw_text("PRESS ENTER TO START", 200.0, 250.0, 32.0, color::WHITE);
        next_frame().await;
        if is_key_down(KeyCode::Enter) {
            break;
        }
    }

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
