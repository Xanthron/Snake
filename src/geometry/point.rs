use std::{
    cmp::PartialEq,
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        return Point { x: x, y: y };
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "x = {}, y = {}", self.x, self.y);
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Point::new(self.x + other.x, self.y + other.y);
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Point::new(self.x - other.x, self.y - other.y);
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        return Point::new(self.x * other, self.y * other);
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        return Point::new(self.x / other, self.y / other);
    }
}

// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         return self.x == other.x && self.y == other.y;
//     }
// }
