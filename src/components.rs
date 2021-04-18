use bevy::math::Vec3;

#[derive(Debug, Clone)]
pub struct Velocity(pub Vec3);

#[derive(Debug, Clone)]
pub struct AngularVelocity(pub f32);

#[derive(Debug, Clone)]
pub struct Player;

#[derive(Debug, Clone)]
pub struct Bullet;

#[derive(Debug, Clone)]
pub struct Rock;

#[derive(Debug, Clone)]
pub struct TimeToLive(pub f32);

#[derive(Debug, Clone)]
pub struct TimeToFire(pub f32);

#[derive(Debug, Clone)]
pub struct Life(pub f32);

#[derive(Debug, Clone)]
pub struct Collideable;

#[derive(Debug, Clone)]
pub struct Size {
    pub x: f32,
    pub y: f32,
}
