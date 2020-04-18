use crate::constants::SHIP_COLOR;
use ggez::{
    graphics,
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};
use log::info;
use std::ops::{Add, Sub};
pub mod ship;

pub type Speed = f32;
pub type Direction = f32;
pub type Mass = f32;
pub type Life = f32;

#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub position: Position,
    pub perimeter: Vec<Position>,
    pub speed: Speed,
    pub direction: Direction,
    pub mass: Mass,
    pub life: Life,
}

impl Object {
    pub fn new(
        name: String,
        position: Position,
        perimeter: Vec<Position>,
        speed: Speed,
        direction: Direction,
        mass: Mass,
        life: Life,
    ) -> Self {
        Self {
            name,
            position,
            perimeter,
            speed,
            direction,
            mass,
            life,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let points: Vec<Point2<f32>> = self
            .perimeter
            .clone()
            .into_iter()
            .map(|position: Position| Point2::from(position))
            .collect();
        let ship_polygon =
            graphics::Mesh::new_polygon(ctx, graphics::DrawMode::stroke(0.5), &points, SHIP_COLOR)?;

        let pos: Point2<f32> = self.position.into();
        let drawparams = graphics::DrawParam::new().dest(pos).rotation(0.0 as f32);

        graphics::draw(ctx, &ship_polygon, drawparams)
    }

    pub fn accelerate(&mut self, qty: f32) {
        info!("Acceleration of {} my dude !", qty);
        self.speed = self.speed + qty;
    }

    pub fn turn(&mut self, qty: f32) {
        info!("Turn of {} my dude !", qty);
        self.direction = self.direction + qty;
    }

    pub fn explode(&mut self) {
        info!("KABOOOOOOM !!!!!!!");
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

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
