use crate::constants::*;
use ggez::{
    graphics,
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};
use log::info;
pub mod ship;

pub type Speed = Vector2<f32>;
pub type Point = Point2<f32>;
pub type Direction = f32;
pub type DirectionVector = Vector2<f32>;
pub type Mass = f32;
pub type Force = f32;
pub type Life = f32;

#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub position: Point,
    pub perimeter: Vec<Point>,
    pub speed: Speed,
    pub direction: Direction,
    pub mass: Mass,
    pub life: Life,
}

impl Object {
    pub fn new(
        name: String,
        position: Point,
        perimeter: Vec<Point>,
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

    pub fn update_position(&mut self, dt: f32) {
        // Clamp the velocity to the max efficiently
        let norm_sq = self.speed.norm_squared();
        if norm_sq > MAX_PHYSICS_VEL.powi(2) {
            self.speed = self.speed / norm_sq.sqrt() * MAX_PHYSICS_VEL;
        }
  
        self.position += self.speed * (dt);
    }

    pub fn accelerate(&mut self, f: Force, dt: f32) {
        info!("Acceleration of {} my dude !", f);

        let direction_vector: DirectionVector = vec_from_angle(-self.direction);
        self.speed += direction_vector * f * dt * self.mass;
    }

    pub fn turn(&mut self, qty: f32, dt: f32) {
        info!("Turn {} my dude !", qty);
        self.direction = self.direction + qty*dt*self.mass;

        
    }

    pub fn explode(&mut self) {
        info!("KABOOOOOOM !!!!!!!");
    }
}

/// Create a unit vector representing the
/// given angle (in radians)
fn vec_from_angle(angle: f32) -> Vector2<f32> {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}
