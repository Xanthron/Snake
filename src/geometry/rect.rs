extern crate piston_window;
use super::border::Border;
use super::point::Point;

#[derive(Copy, Clone)]
pub struct Rect {
    pub x_min: i32,
    pub y_min: i32,
    pub x_max: i32,
    pub y_max: i32,
}

impl Rect {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Rect {
        let (x_min, x_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y_min, y_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        return Rect {
            x_min: x_min,
            y_min: y_min,
            x_max: x_max,
            y_max: y_max,
        };
    }

    pub fn from_border(self, border: Border) -> Rect {
        return Rect::new(
            self.x_min + border.left,
            self.y_min + border.top,
            self.x_max - border.right,
            self.y_max - border.bottom,
        );
    }

    pub fn contains_point(self, point: Point) -> bool {
        return self.x_min <= point.x && self.x_max > point.x && self.y_min <= point.y && self.y_max > point.y;
    }
}
