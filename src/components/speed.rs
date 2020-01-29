use amethyst::{
    core::{math::base::Vector3},
    ecs::prelude::{Component, DenseVecStorage},
};

#[derive(Debug)]
pub struct SpeedComponent {
    pub speed: Vector3<f32>,
}

impl SpeedComponent {
    pub fn new(speed: Vector3<f32>) -> SpeedComponent {
        SpeedComponent { speed }
    }
}

impl Component for SpeedComponent {
    type Storage = DenseVecStorage<Self>;
}
