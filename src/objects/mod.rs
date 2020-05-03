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

pub trait Moveable {
    fn get_position(&self) -> &Point;
    fn get_position_mut(&mut self) -> &mut Point;
    fn set_position(&mut self, position: Point);

    fn get_speed(&self) -> &SpeedVector;
    fn set_speed(&mut self, speed: SpeedVector);
    fn get_max_speed(&self) -> &Speed;
    fn set_max_speed(&mut self, max_speed: Speed);

    fn get_direction(&self) -> &Direction;
    fn set_direction(&mut self, direction: Direction);

    fn get_angle_speed(&self) -> &SpeedAngle;
    fn set_angle_speed(&mut self, angle_speed: SpeedAngle);
    fn get_max_angle_speed(&self) -> &SpeedAngle;
    fn set_max_angle_speed(&mut self, max_angle_speed: SpeedAngle);

    fn get_mass(&self) -> &Mass;
    fn set_mass(&mut self, mass: Mass);

    fn update_position(&mut self, dt: f32) {
        // speed
        self.set_position(self.get_position() + self.get_speed() * (dt));

        if self.get_position().x > GAME_MAX_WIDTH || self.get_position().x < -GAME_MAX_WIDTH {
            self.get_position_mut().x = -self.get_position().x;
        }
        if self.get_position().y > GAME_MAX_HEIGHT || self.get_position().y < -GAME_MAX_HEIGHT {
            self.get_position_mut().y = -self.get_position().y;
        }

        // angular speed
        self.set_direction(self.get_direction() + self.get_angle_speed() * (dt));
    }

    /// Create a unit vector representing the
    /// given angle (in radians)
    fn vec_from_angle(angle: f32) -> Vector2<f32> {
        let vx = angle.sin();
        let vy = angle.cos();
        Vector2::new(vx, vy)
    }
}

pub trait Playable: Moveable {
    fn update_speeds(&mut self, dt: f32) {
        // Apply descelerations to ship
        self.set_speed(self.get_speed() - self.get_speed() * dt * SHIP_DESCELERATION);
        self.set_angle_speed(
            self.get_angle_speed() - self.get_angle_speed() * dt * SHIP_ANGLE_DESCELERATION,
        );
    }

    fn accelerate(&mut self, f: Force, dt: f32) {
        debug!("Acceleration of {} my dude !", f);

        let direction_vector: DirectionVector = Self::vec_from_angle(self.get_direction().clone());
        self.set_speed(self.get_speed() + direction_vector * f * dt / self.get_mass().clone());

        let norm_sq = self.get_speed().norm_squared();
        if norm_sq > self.get_max_speed().powi(2) {
            self.set_speed(self.get_speed().normalize() * self.get_max_speed().clone());
        }
    }

    fn turn(&mut self, f: f32, dt: f32) {
        debug!("Turn {} my dude !", f);
        self.set_angle_speed(self.get_angle_speed() + f * dt / self.get_mass());

        if self.get_angle_speed() > self.get_max_angle_speed() {
            self.set_angle_speed(self.get_max_angle_speed().clone());
        }
        if self.get_angle_speed() < &-self.get_max_angle_speed() {
            self.set_angle_speed(-self.get_max_angle_speed());
        }
    }
}
pub trait Collideable: Moveable {
    fn get_hitbox(&self) -> &HitBox;
    fn set_hitbox(&mut self, hitbox: HitBox);

    fn has_collided_with(&self, other: &Self) -> bool {
        self.get_position().x + self.get_hitbox().width / 2.0
            > other.get_position().x - other.get_hitbox().width / 2.0
            && self.get_position().x - self.get_hitbox().width / 2.0
                < other.get_position().x + other.get_hitbox().width / 2.0
            && self.get_position().y + self.get_hitbox().height / 2.0
                > other.get_position().y - other.get_hitbox().height / 2.0
            && self.get_position().y - self.get_hitbox().height / 2.0
                < other.get_position().y + other.get_hitbox().height / 2.0
    }

    fn compute_speed_vectors_after_collision(
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
}

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    position: Point,
    pub mesh: Mesh,
    speed: SpeedVector,
    max_speed: Speed,
    direction: Direction,
    angle_speed: SpeedAngle,
    max_angle_speed: SpeedAngle,
    mass: Mass,
    pub life: Life,
    hitbox: HitBox,
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
        hitbox: HitBox,
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
            hitbox,
        }
    }

    pub fn explode(&mut self) {
        info!("KABOOOOOOM !!!!!!!");
    }
}

impl Moveable for Object {
    fn get_position(&self) -> &Point {
        &self.position
    }
    fn get_position_mut(&mut self) -> &mut Point {
        &mut self.position
    }
    fn set_position(&mut self, position: Point) {
        self.position = position
    }
    fn get_speed(&self) -> &SpeedVector {
        &self.speed
    }
    fn set_speed(&mut self, speed: SpeedVector) {
        self.speed = speed
    }
    fn get_max_speed(&self) -> &Speed {
        &self.max_speed
    }
    fn set_max_speed(&mut self, max_speed: Speed) {
        self.max_speed = max_speed
    }
    fn get_direction(&self) -> &Direction {
        &self.direction
    }
    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
    fn get_angle_speed(&self) -> &SpeedAngle {
        &self.angle_speed
    }
    fn set_angle_speed(&mut self, angle_speed: SpeedAngle) {
        self.angle_speed = angle_speed
    }
    fn get_max_angle_speed(&self) -> &SpeedAngle {
        &self.max_angle_speed
    }
    fn set_max_angle_speed(&mut self, max_angle_speed: SpeedAngle) {
        self.max_angle_speed = max_angle_speed
    }
    fn get_mass(&self) -> &Mass {
        &self.mass
    }
    fn set_mass(&mut self, mass: Mass) {
        self.mass = mass
    }
}

impl Playable for Object {}

impl Collideable for Object {
    fn get_hitbox(&self) -> &HitBox {
        &self.hitbox
    }
    fn set_hitbox(&mut self, hitbox: HitBox) {
        self.hitbox = hitbox
    }
}
