use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH, SHIP_SIZE_X, SHIP_SIZE_Y};
use ggez::nalgebra::{Point2, Vector2};
use std::ops::Add;

#[derive(Debug)]
pub struct Ship {
    pub position: Position,
    pub perimeter: [Point2<f32>; 4],
}

impl Ship {
    pub fn new() -> Ship {
        let position_x = ARENA_WIDTH / 2.0;
        let position_y = ARENA_HEIGHT / 2.0;

        Ship {
            position: Position::new(position_x, position_y),
            perimeter: [
                Point2::new(-SHIP_SIZE_X, -SHIP_SIZE_Y),
                Point2::new(0.0, SHIP_SIZE_Y),
                Point2::new(SHIP_SIZE_X, -SHIP_SIZE_Y),
                Point2::new(0.0, 0.0),
            ],
        }
    }

    pub fn move_forward(&mut self, qty: f32) {
        self.position.y = self.position.y + qty;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }
}

impl From<Point2<f32>> for Position {
    fn from(point: Point2<f32>) -> Self {
        Position::new(point.x, point.y)
    }
}

impl From<Position> for Point2<f32> {
    fn from(position: Position) -> Self {
        Point2::new(position.x, position.y)
    }
}

impl From<Vector2<f32>> for Position {
    fn from(vector: Vector2<f32>) -> Self {
        Position::new(vector.x, vector.y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
