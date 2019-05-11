use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct SimpleBulletComponent {}

impl SimpleBulletComponent {
    pub fn new() -> SimpleBulletComponent {
        SimpleBulletComponent {}
    }
}

impl Component for SimpleBulletComponent {
    type Storage = DenseVecStorage<Self>;
}
