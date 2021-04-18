use std::f32::consts::TAU;

use bevy::{
    math::{Vec2, Vec3},
    prelude::*,
    prelude::{Assets, Color, Transform},
    sprite::ColorMaterial,
};
use bevy_prototype_lyon::prelude::*;

use crate::{
    components::{Size, *},
    constants::*,
};

#[derive(Bundle)]
pub struct ShipEntity {
    player: Player,
    velocity: Velocity,
    time_to_fire: TimeToFire,
    life: Life,
    collideable: Collideable,
    size: Size,
}

impl<'a> ShipEntity {
    pub fn new(velocity: Velocity) -> Self {
        Self {
            player: Player,
            velocity,
            time_to_fire: TimeToFire(SHIP_RELOAD_TIME),
            life: Life(SHIP_LIFE),
            collideable: Collideable,
            size: Size {
                x: 2. * SHIP_SIZE_X,
                y: 2. * SHIP_SIZE_Y,
            },
        }
    }

    pub fn spawn_ship(self, commands: &'a mut Commands) {
        let ship = shapes::Polygon {
            points: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(-SHIP_SIZE_X, SHIP_SIZE_Y),
                Vec2::new(SHIP_SIZE_X, 0.0),
                Vec2::new(-SHIP_SIZE_X, -SHIP_SIZE_Y),
            ],
            closed: true,
        };

        let transform = Transform {
            translation: Vec3::ZERO,
            rotation: Quat::from_axis_angle(Vec3::Z, TAU / 4.0),
            ..Default::default()
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &ship,
                ShapeColors::new(Color::GREEN),
                DrawMode::Stroke(StrokeOptions::default()),
                transform,
            ))
            .insert_bundle(self);
    }
}
