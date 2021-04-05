use bevy::math::Vec3;

pub struct Paddle {
    pub speed: f32,
}

pub struct Ball {
    pub velocity: Vec3,
}

pub struct Scoreboard {
    pub score: usize,
}

pub enum Collider {
    Solid,
    Scorable,
    Paddle,
}
