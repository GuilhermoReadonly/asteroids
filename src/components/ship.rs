use amethyst::{
    core::{math::base::Vector3, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};

use crate::asteroids::{Assets, ARENA_HEIGHT, ARENA_WIDTH};
use crate::components::SpeedComponent;

#[derive(Debug)]
pub struct ShipComponent {}

impl ShipComponent {
    pub fn new() -> ShipComponent {
        ShipComponent {}
    }

    pub fn spawn_ship(world: &mut World) {
        let assets = world.read_resource::<Assets>().clone();

        // Correctly position the ship in the middle of the arena.
        let y = ARENA_HEIGHT / 2.0;
        let x = ARENA_WIDTH / 2.0;
        let mut ship_transform = Transform::default();
        ship_transform.set_translation_xyz(x, y, 0.0).set_scale(Vector3::new(1.0, 1.0, 1.0));

        // Create a ship entity.
        world
            .create_entity()
            .with(assets.ship.clone())
            .with(assets.color.clone())
            .with(ShipComponent::new())
            .with(SpeedComponent::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(ship_transform)
            .build();
    }
}

impl Component for ShipComponent {
    type Storage = DenseVecStorage<Self>;
}
