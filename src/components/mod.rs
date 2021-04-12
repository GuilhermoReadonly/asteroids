use bevy::math::Vec2;

#[derive(Debug, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Clone)]
pub struct Rotation(pub f32);

#[derive(Debug, Clone)]
pub struct Ship;

#[derive(Debug, Clone)]
pub struct Bullet;

#[derive(Debug, Clone)]
pub struct TimeToLive(pub f32);

#[derive(Debug, Clone)]
pub struct TimeToFire(pub f32);
