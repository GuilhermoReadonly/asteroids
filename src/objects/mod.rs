pub mod bullet;
pub mod hit_box;
pub mod rock;
pub mod ship;
pub mod star;

use crate::{constants::*, objects::hit_box::HitBox};
use ggez::{
    graphics::Mesh,
    nalgebra::{Point2, Vector2},
};
use log::{debug, info};

pub type SpeedVector = Vector2<f32>;
pub type Speed = f32;
pub type SpeedAngle = f32;
pub type PositionVector = Vector2<f32>;
pub type Point = Point2<f32>;
pub type Direction = f32;
pub type DirectionVector = Vector2<f32>;
pub type Mass = f32;
pub type Force = f32;
pub type Life = f32;

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub position: Point,
    pub mesh: Mesh,
    pub speed: SpeedVector,
    pub max_speed: Speed,
    pub direction: Direction,
    pub angle_speed: SpeedAngle,
    pub max_angle_speed: SpeedAngle,
    pub mass: Mass,
    pub life: Life,
    pub hit_box: HitBox,
}

impl Object {
    pub fn new(
        name: String,
        position: Point,
        mesh: Mesh,
        speed: SpeedVector,
        max_speed: Speed,
        direction: Direction,
        angle_speed: SpeedAngle,
        max_angle_speed: SpeedAngle,
        mass: Mass,
        life: Life,
        hit_box: HitBox,
    ) -> Self {
        Self {
            name,
            position,
            mesh: mesh,
            speed,
            max_speed,
            direction,
            angle_speed,
            max_angle_speed,
            mass,
            life,
            hit_box,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        // speed
        self.position += self.speed * (dt);

        if self.position.x > GAME_MAX_WIDTH || self.position.x < -GAME_MAX_WIDTH {
            self.position.x = -self.position.x;
        }
        if self.position.y > GAME_MAX_HEIGHT || self.position.y < -GAME_MAX_HEIGHT {
            self.position.y = -self.position.y;
        }

        // angular speed
        self.direction += self.angle_speed * (dt);
    }

    pub fn update_speeds(&mut self, dt: f32) {
        // Apply descelerations to ship
        self.speed = self.speed - self.speed * dt * SHIP_DESCELERATION;
        self.angle_speed = self.angle_speed - self.angle_speed * dt * SHIP_ANGLE_DESCELERATION;
    }

    pub fn accelerate(&mut self, f: Force, dt: f32) {
        debug!("Acceleration of {} my dude !", f);

        let direction_vector: DirectionVector = vec_from_angle(self.direction);
        self.speed += direction_vector * f * dt / self.mass;

        let norm_sq = self.speed.norm_squared();
        if norm_sq > self.max_speed.powi(2) {
            self.speed = self.speed.normalize() * self.max_speed;
        }
    }

    pub fn turn(&mut self, f: f32, dt: f32) {
        debug!("Turn {} my dude !", f);
        self.angle_speed += f * dt / self.mass;

        if self.angle_speed > self.max_angle_speed {
            self.angle_speed = self.max_angle_speed;
        }
        if self.angle_speed < -self.max_angle_speed {
            self.angle_speed = -self.max_angle_speed;
        }
    }

    pub fn has_collided_with(&self, other: &Self) -> bool {
        self.position.x + self.hit_box.width/2.0 > other.position.x - other.hit_box.width/2.0 &&
        self.position.x - self.hit_box.width/2.0 < other.position.x + other.hit_box.width/2.0 &&
        self.position.y + self.hit_box.height/2.0 > other.position.y - other.hit_box.height/2.0 &&
        self.position.y - self.hit_box.height/2.0 < other.position.y + other.hit_box.height/2.0 
    }

    pub fn compute_speed_vector_after_collision(
        v1: SpeedVector,
        v2: SpeedVector,
        m1: f32,
        m2: f32,
        x1: PositionVector,
        x2: PositionVector,
    ) -> (SpeedVector, SpeedVector) {
        let speed1: SpeedVector = v1
            - (2.0 * m2) / (m1 + m2) * ((v1 - v2).dot(&(x1 - x2))) / ((x1 - x2).dot(&(x1 - x2)))
                * (x1 - x2);
        let speed2: SpeedVector = v2
            - (2.0 * m1) / (m2 + m1) * ((v2 - v1).dot(&(x2 - x1))) / ((x2 - x1).dot(&(x2 - x1)))
                * (x2 - x1);
        (speed1, speed2)
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
