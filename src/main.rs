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

fn conf() -> Conf {
    Conf {
        // Il titolo che apparirà nella barra della finestra
        window_title: "Snakers".to_string(),
        // La larghezza desiderata per la finestra di gioco
        window_width: WIDTH as i32,
        // L'altezza desiderata per la finestra di gioco
        window_height: HEIGHT as i32,
        // Opzionale: puoi rendere la finestra non ridimensionabile dall'utente
        window_resizable: false,
        ..Default::default() // Usa i valori di default per tutte le altre opzioni
    }
}

#[macroquad::main(conf)]
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
