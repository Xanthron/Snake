use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul, Sub},
};
pub struct Vector_2d {
    pub x: f64,
    pub y: f64,
}
impl Display for Vector_2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f, "(x = {}, y = {})", self.x, self.y);
    }
}
impl Add for Vector_2d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self::new(self.x + other.x, self.y + other.y);
    }
}
impl Sub for Vector_2d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Self::new(self.x - other.x, self.y - other.y);
    }
}
impl Mul<f64> for Vector_2d {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        return Self::new(self.x * other, self.y * other);
    }
}
impl Vector_2d {
    pub fn new(x: f64, y: f64) -> Vector_2d {
        return Vector_2d { x: x, y: y };
    }
    pub fn sqr_magnitude(&self) -> f64 {
        return self.x.powi(2) + self.y.powi(2);
    }
    pub fn magnitude(&self) -> f64 {
        return self.sqr_magnitude().sqrt();
    }
    pub fn normalize(&self) -> Vector_2d {
        let magnitude = self.magnitude();
        return Vector_2d::new(self.x / magnitude, self.y / magnitude);
    }
    pub fn angle(from: Vector_2d, to: Vector_2d) -> f64 {
        let from_sin_cos = (from.x / from.y).sin_cos();
        let to_sin_cos = (to.x / to.y).sin_cos();
        return from_sin_cos.0 / from_sin_cos.1 - to_sin_cos.0 / to_sin_cos.1;
    }
    pub fn distance(from: Vector_2d, to: Vector_2d) -> f64 {
        return (to.magnitude() - from.magnitude()).abs();
    }
}
