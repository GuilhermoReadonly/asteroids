use crate::constants::*;
use ggez::{
    graphics::Mesh,
    nalgebra::{Point2, Vector2},
};
use log::{debug, info};

pub mod bullet;
pub mod rock;
pub mod ship;

pub type SpeedVector = Vector2<f32>;
pub type Speed = f32;
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
    pub mesh: Mesh,
    pub speed: SpeedVector,
    pub max_speed: Speed,
    pub direction: Direction,
    pub mass: Mass,
    pub life: Life,
}

impl Object {
    pub fn new(
        name: String,
        position: Point,
        mesh: Mesh,
        speed: SpeedVector,
        max_speed: Speed,
        direction: Direction,
        mass: Mass,
        life: Life,
    ) -> Self {
        Self {
            name,
            position,
            mesh: mesh,
            speed,
            max_speed,
            direction,
            mass,
            life,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        // Clamp the velocity to the max efficiently
        let norm_sq = self.speed.norm_squared();
        if norm_sq > self.max_speed.powi(2) {
            self.speed = self.speed / norm_sq.sqrt() * self.max_speed;
        }
        self.position += self.speed * (dt);

        if self.position.x > GAME_MAX_WIDTH || self.position.x < -GAME_MAX_WIDTH {
            self.position.x = -self.position.x;
        }
        if self.position.y > GAME_MAX_HEIGHT || self.position.y < -GAME_MAX_HEIGHT {
            self.position.y = -self.position.y;
        }
    }

    pub fn accelerate(&mut self, f: Force, dt: f32) {
        debug!("Acceleration of {} my dude !", f);

        let direction_vector: DirectionVector = vec_from_angle(self.direction);
        self.speed += direction_vector * f * dt * self.mass;
    }

    pub fn turn(&mut self, qty: f32, dt: f32) {
        debug!("Turn {} my dude !", qty);
        self.direction = self.direction + qty * dt * self.mass;
    }

    pub fn explode(&mut self) {
        info!("KABOOOOOOM !!!!!!!");
    }
}

/// Create a unit vector representing the
/// given angle (in radians)
pub fn vec_from_angle(angle: f32) -> Vector2<f32> {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}
