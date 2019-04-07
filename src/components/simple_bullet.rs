use amethyst::{
    core::nalgebra::base::Vector3,
    ecs::prelude::{Component, DenseVecStorage},
};

#[derive(Debug)]
pub struct SimpleBulletComponent {
    pub speed: Vector3<f32>,
}

impl SimpleBulletComponent {
    pub fn new(speed: Vector3<f32>) -> SimpleBulletComponent {
        SimpleBulletComponent { speed }
    }
}

impl Component for SimpleBulletComponent {
    type Storage = DenseVecStorage<Self>;
}
