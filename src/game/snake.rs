extern crate piston_window;
use super::*;
use crate::geometry::*;
use piston_window::{types::Color, Context, G2d};

const SNAKE_BODY_COLOR: Color = [0.0, 1.0, 0.0, 1.0];
const SNAKE_HEAD_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

pub struct Snake {
    points: Vec<Point>,
    direction: Direction,
}

impl Snake {
    pub(crate) fn new(game_rect: Rect, dir: Direction) -> Snake {
        let x_m = (game_rect.x_max + game_rect.x_min) / 2;
        let y_m = (game_rect.y_max + game_rect.y_min) / 2;
        let mut points = Vec::<Point>::new();
        for i in 0..4 {
            points.push(Point::new(x_m, y_m - 4 / 2 + i))
        }

        return Snake {
            points: points,
            direction: dir,
        };
    }

    pub fn get_head(&self) -> Point {
        return self.points[0];
    }
    pub fn move_dir(&mut self, dir: Direction) {
        let dir = if dir == self.direction.get_opposite() {
            self.direction
        } else {
            dir
        };

        let current = self.points[0];
        let mut next = current + dir.get_direction_point();

        for i in 0..self.points.len() {
            let temp = self.points[i];
            self.points[i] = next;
            next = temp;
        }

        self.direction = dir;
    }

    pub fn eat_apple(&mut self) {
        let last = self.points[self.points.len() - 1];
        self.points.push(Point::new(last.x, last.y));
    }

    pub fn check_head_eat_self(&self) -> bool {
        let head = self.points[0];
        for i in 1..self.points.len() {
            if self.points[i] == head {
                println!("Eat Self");
                return true;
            }
        }
        return false;
    }

    pub fn check_head_out_of_boarder(&self, game_board: &Rect) -> bool {
        return !game_board.contains_point(self.get_head());
    }

    pub fn draw(&self, context: &Context, g: &mut G2d) {
        for (i, point) in self.points.iter().enumerate() {
            if i == 0 {
                draw::point(SNAKE_HEAD_COLOR, point, context, g);
            } else {
                draw::point(SNAKE_BODY_COLOR, point, context, g);
            }
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        for p in &self.points {
            if point == *p {
                return true;
            }
        }
        return false;
    }
}
