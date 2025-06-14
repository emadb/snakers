use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::{Button, Key, RenderArgs, UpdateArgs};

use crate::food::Food;
use crate::position::Position;
use crate::snake::{Direction, Snake, SnakeStates};
use graphics::*;

pub struct App {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub food: Food,
    pub font: GlyphCache<'static>,
    pub walls: Vec<Position>,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        match self.snake.state {
            SnakeStates::Alive => {
                self.gl
                    .draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
                        clear(BACKGROUND, gl);

                        draw_walls(&self.walls[..], &c, gl);
                        draw_points(&self.snake.len, &mut self.font, &c, gl);
                        draw_food(&self.food, &c, gl);
                        draw_snake(&self.snake, &c, gl);
                    });
            }
            SnakeStates::SelfEaten => self.gl.draw(args.viewport(), |c, gl| {
                clear(BACKGROUND, gl);
                draw_game_over(&mut self.font, &c, gl);
            }),
            SnakeStates::Smashed => self.gl.draw(args.viewport(), |c, gl| {
                clear(BACKGROUND, gl);
                draw_game_over(&mut self.font, &c, gl);
            }),
        }
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.snake.next();

        // Check food
        if self.snake.head == self.food.pos {
            self.snake.grow();
            self.food.next_pos();
        }

        // Check walls
        for w in &self.walls {
            if self.snake.head == *w {
                self.snake.state = SnakeStates::Smashed;
            }
        }
    }

    pub fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => change_direction(&mut self.snake, Direction::South, Direction::North),
                Key::Down => change_direction(&mut self.snake, Direction::North, Direction::South),
                Key::Left => change_direction(&mut self.snake, Direction::East, Direction::West),
                Key::Right => change_direction(&mut self.snake, Direction::West, Direction::East),
                _ => {}
            }
        }
    }
}

fn change_direction(snake: &mut Snake, from: Direction, to: Direction) {
    if snake.direction != from {
        snake.direction = to
    } else {
        snake.state = SnakeStates::SelfEaten;
    }
}

fn draw_food(food: &Food, c: &Context, gl: &mut GlGraphics) {
    const FOOD: [f32; 4] = [0.9, 0.1, 0.1, 1.0];
    let food = rectangle::square(food.pos.0, food.pos.1, 10.0);
    rectangle(FOOD, food, c.transform.trans(0.0, 0.0), gl);
}

fn draw_walls(walls: &[Position], c: &Context, gl: &mut GlGraphics) {
    const WALL: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
    for w in walls {
        rectangle(
            WALL,
            rectangle::square(w.0, w.1, 10.0),
            c.transform.trans(0.0, 0.0),
            gl,
        );
    }
}

fn draw_points(points: &u32, font: &mut GlyphCache<'static>, c: &Context, gl: &mut GlGraphics) {
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    text(
        WHITE,
        32,
        points.to_string().as_str(),
        font,
        c.transform.trans(10.0, 50.0),
        gl,
    )
    .unwrap();
}

fn draw_snake(snake: &Snake, c: &Context, gl: &mut GlGraphics) {
    const SNAKE: [f32; 4] = [0.0, 0.4, 1.0, 1.0];
    let Position(x, y) = snake.head;
    let snake_shape = rectangle::square(x, y, 10.0);
    rectangle(SNAKE, snake_shape, c.transform.trans(0.0, 0.0), gl);
    for sn in &snake.tail {
        let Position(xs, ys) = sn;
        let t = rectangle::square(*xs, *ys, 10.0);
        rectangle(SNAKE, t, c.transform.trans(0.0, 0.0), gl);
    }
}

fn draw_game_over(font: &mut GlyphCache<'static>, c: &Context, gl: &mut GlGraphics) {
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    text(
        WHITE,
        32,
        "GAME OVER :-(",
        font,
        c.transform.trans(200.0, 250.0),
        gl,
    )
    .unwrap();
}
