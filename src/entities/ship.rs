use bevy::{
    ecs::{Commands, ResMut},
    math::{Vec2, Vec3},
    prelude::*,
    prelude::{Assets, Color, Transform},
    sprite::ColorMaterial,
};
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, StrokeOptions, TessellationMode},
    shapes,
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
    ) -> &'a mut Commands {
        let ship = shapes::Polygon {
            points: vec![
                Vec2::new(SHIP_SIZE_X, -SHIP_SIZE_Y),
                Vec2::new(0.0, SHIP_SIZE_Y),
                Vec2::new(-SHIP_SIZE_X, -SHIP_SIZE_Y),
                Vec2::new(0.0, 0.0),
            ],
            closed: true,
        };

        commands
            .spawn(GeometryBuilder::build_as(
                &ship,
                materials.add(ColorMaterial::color(Color::GREEN)),
                TessellationMode::Stroke(StrokeOptions::default()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ))
            .with_bundle(self)
    }
}
