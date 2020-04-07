extern crate piston_window;
use crate::geometry::{Point, Rect};

pub const SCALE: i32 = 20;

pub fn rectangle(
    color: piston_window::types::Color,
    rect: &Rect,
    context: &piston_window::Context,
    g: &mut piston_window::G2d,
) {
    let rect = [
        (rect.x_min * SCALE) as f64,
        (rect.y_min * SCALE) as f64,
        ((rect.x_max - rect.x_min) * SCALE) as f64,
        ((rect.y_max - rect.y_min) * SCALE) as f64,
    ];
    piston_window::rectangle(color, rect, context.transform, g);
}

pub fn point(
    color: piston_window::types::Color,
    point: &Point,
    context: &piston_window::Context,
    g: &mut piston_window::G2d,
) {
    let rect = [
        (point.x * SCALE) as f64,
        (point.y * SCALE) as f64,
        (SCALE) as f64,
        (SCALE) as f64,
    ];
    piston_window::rectangle(color, rect, context.transform, g);
}
