use crate::food::Food;
use crate::position::Position;
use crate::snake::{Direction, Snake, SnakeStates};
use crate::{HEIGHT, WIDTH};
use macroquad::color;
use macroquad::input::is_key_down;
use macroquad::prelude::KeyCode;
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use rand::Rng;

pub struct App {
    pub snake: Snake,
    pub food: Food,
    pub walls: Vec<Position>,
}

impl App {
    pub fn new() -> App {
        let mut rng = rand::rng();

        let mut walls: Vec<Position> = vec![];
        for _ in 1..20 {
            walls.push(Position::new(
                (rng.random_range(0..60) * crate::STEP) as f32,
                (rng.random_range(0..60) * crate::STEP) as f32,
            ));
        }

        App {
            snake: Snake::new(Position::new(100.0, 100.0), Direction::East),
            food: Food {
                pos: Position::new(500.0, 500.0),
                count: 1,
            },
            walls,
        }
    }

    pub fn render(&mut self) {
        match self.snake.state {
            SnakeStates::Alive => {
                draw_borders();
                draw_walls(&self.walls[..]);
                draw_points(&self.snake.len);
                draw_food(&self.food);
                draw_snake(&self.snake);
            }
            SnakeStates::SelfEaten => {
                draw_game_over();
            }
            SnakeStates::Smashed => {
                draw_game_over();
            }
        }
    }

    pub fn update(&mut self) {
        self.snake.next();
        check_food(&mut self.snake, &mut self.food);
        check_walls(&mut self.snake, &self.walls);
        check_position(&mut self.snake);
    }

    pub fn handle_keys(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.snake
                .change_direction(Direction::South, Direction::North);
        }

        if is_key_down(KeyCode::Down) {
            self.snake
                .change_direction(Direction::North, Direction::South);
        }
        if is_key_down(KeyCode::Left) {
            self.snake
                .change_direction(Direction::East, Direction::West);
        }
        if is_key_down(KeyCode::Right) {
            self.snake
                .change_direction(Direction::West, Direction::East);
        }
    }
}

fn check_food(snake: &mut Snake, food: &mut Food) {
    if snake.head == food.pos {
        snake.grow();
        food.create_new();
    }
}

fn check_walls(snake: &mut Snake, walls: &Vec<Position>) {
    for w in walls {
        if snake.head == *w {
            snake.smash();
        }
    }
}

fn check_position(snake: &mut Snake) {
    // if snake.head.0 < 0.0 || snake.head.0 > crate::WIDTH {
    //     println!("OUT OF SCREEN (W)");
    //     snake.smash();
    // }
    // if snake.head.1 < 0.0 || snake.head.1 > crate::HEIGHT {
    //     println!("OUT OF SCREEN (H)");
    //     snake.smash();
    // }
}
fn draw_borders() {
    // draw_rectangle(0.0, 0.0, WIDTH, HEIGHT, color::GRAY);
}
fn draw_food(food: &Food) {
    draw_rectangle(food.pos.0, food.pos.1, 10.0, 10.0, color::RED);
}

fn draw_walls(walls: &[Position]) {
    for w in walls {
        draw_rectangle(w.0, w.1, 10.0, 10.0, color::YELLOW);
    }
}

fn draw_points(points: &u32) {
    draw_text(points.to_string().as_str(), 5.0, 50.0, 32.0, color::WHITE);
}

fn draw_snake(snake: &Snake) {
    let Position(x, y) = snake.head;

    draw_rectangle(x, y, 10.0, 10.0, color::SKYBLUE);

    for sn in &snake.tail {
        let Position(xs, ys) = sn;
        draw_rectangle(*xs, *ys, 10.0, 10.0, color::SKYBLUE);
    }
}

fn draw_game_over() {
    draw_text("GAME OVER :-(", 200.0, 250.0, 32.0, color::WHITE);
}
