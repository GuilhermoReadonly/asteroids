use bevy::{
    math::{Vec2, Vec3},
    prelude::*,
    prelude::{Assets, Color, Transform},
    sprite::ColorMaterial,
};

use crate::{
    components::{Rotation, Ship, Velocity},
    constants::{SHIP_SIZE_X, SHIP_SIZE_Y},
};

#[derive(Bundle)]
pub struct ShipEntity {
    ship: Ship,
    velocity: Velocity,
    rotation: Rotation,
}

impl<'a> ShipEntity {
    pub fn new(velocity: Velocity, rotation: Rotation) -> Self {
        Self {
            ship: Ship {},
            velocity,
            rotation,
        }
    }

    pub fn spawn_ship(
        self,
        commands: &'a mut Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let ship = vec![
            Vec2::new(SHIP_SIZE_X, -SHIP_SIZE_Y),
            Vec2::new(0.0, SHIP_SIZE_Y),
            Vec2::new(-SHIP_SIZE_X, -SHIP_SIZE_Y),
            Vec2::new(0.0, 0.0),
        ];

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.0, 0.1, 0.0).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                sprite: Sprite::new(Vec2::new(30.0, 30.0)),
                ..Default::default()
            })
            .insert_bundle(self);
    }
}
