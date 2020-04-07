extern crate piston_window;
use piston_window::{types::Color, Key};

use super::*;
use crate::geometry::*;
use rand::Rng;

const APPLE_COLOR: Color = [0.9, 0.1, 0.1, 1.0];
const BORDER_COLOR: Color = [0.1, 0.1, 0.15, 1.0];
const BOARD_COLOR: Color = [0.9, 0.9, 0.95, 1.0];
const GAME_OVER: Color = [1.0, 0.0, 0.0, 0.2];
const UPDATE_TIME: f64 = 0.25;

pub struct Board {
    apple_position: Point,
    snake: Snake,
    rect: Rect,
    game_rect: Rect,
    last_press: Direction,
    next_move: f64,
    game_over: bool,
}

impl Board {
    pub fn new(rect: Rect, border: Border) -> Board {
        let game_rect = rect.from_border(border);
        let dir = Direction::Up;
        let mut board = Board {
            apple_position: Point::new(0, 0),
            snake: Snake::new(game_rect, dir),
            //border: border,
            rect: rect,
            game_rect: game_rect,
            last_press: dir,
            next_move: UPDATE_TIME,
            game_over: false,
        };
        board.place_apple();
        return board;
    }
    pub fn place_apple(&mut self) {
        let mut rand = rand::thread_rng();
        let x = rand.gen_range(self.game_rect.x_min, self.game_rect.x_max);
        let y = rand.gen_range(self.game_rect.y_min, self.game_rect.y_max);
        if self.snake.contains_point(Point::new(x, y)) {
            self.place_apple();
        } else {
            self.apple_position.x = x;
            self.apple_position.y = y;
        }
    }
    fn move_snack(&mut self) -> bool {
        let snake = &mut self.snake;
        snake.move_dir(self.last_press);

        if snake.check_head_eat_self() || snake.check_head_out_of_boarder(&self.game_rect) {
            self.next_move = 3.0;
            self.game_over = true;
            return false;
        }
        let head = snake.get_head();
        //println!("{:?} {:?}", head, self.apple_position);
        if head == self.apple_position {
            snake.eat_apple();
            self.place_apple();
        }
        true
    }
    pub fn draw(&self, context: &piston_window::Context, g: &mut piston_window::G2d) {
        draw::rectangle(BORDER_COLOR, &self.rect, context, g);
        draw::rectangle(BOARD_COLOR, &self.game_rect, context, g);
        draw::point(APPLE_COLOR, &self.apple_position, context, g);
        self.snake.draw(context, g);
        if self.game_over {
            draw::rectangle(GAME_OVER, &self.rect, context, g);
        }
    }

    pub fn key_press(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        if let Some(dir) = dir {
            self.last_press = dir;
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.next_move -= delta_time;
        if self.game_over {
            if self.next_move <= 0.0 {
                self.reset()
            }
        } else {
            if self.next_move <= 0.0 {
                self.next_move = UPDATE_TIME;
                self.move_snack();
            }
        }
    }
    pub fn reset(&mut self) {
        self.last_press = Direction::Up;
        self.snake = Snake::new(self.game_rect, Direction::Up);
        self.place_apple();
        self.game_over = false;
    }
}
