use std::cmp::PartialEq;

pub const WORLD_WIDTH: f64 = 800.0;
pub const WORLD_HEIGHT: f64 = 600.0;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub trait Positionable {
    fn set_position(&mut self, x: f64, y: f64);

    fn position(&self) -> &Position;
}
