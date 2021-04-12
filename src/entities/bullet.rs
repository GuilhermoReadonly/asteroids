use bevy::{
    math::{Vec2, Vec3},
    prelude::*,
    prelude::{Assets, Color, Transform},
    sprite::ColorMaterial,
};
use bevy_prototype_lyon::prelude::*;

use crate::{components::*, constants::*};

#[derive(Bundle)]
pub struct BulletEntity {
    bullet: Bullet,
    velocity: Velocity,
    rotation: Rotation,
    time_to_live: TimeToLive,
}

impl<'a> BulletEntity {
    pub fn new(rotation: Rotation) -> Self {
        Self {
            bullet: Bullet {},
            velocity: Velocity(BULLET_SPEED * Vec2::new(rotation.0.cos(), rotation.0.sin())),
            rotation,
            time_to_live: TimeToLive(BULLET_LIFE),
        }
    }

    pub fn spawn_bullet(self, commands: &'a mut Commands, position: Vec3) {
        let bullet = shapes::Line(Vec2::new(0.0, 0.0), Vec2::new(BULLET_SIZE, 0.0));

        let transform = Transform {
            translation: position,
            rotation: Quat::from_axis_angle(Vec3::Z, self.rotation.0),
            ..Default::default()
        };

        commands
            .spawn_bundle(self)
            .insert_bundle(GeometryBuilder::build_as(
                &bullet,
                ShapeColors::new(Color::GREEN),
                DrawMode::Stroke(StrokeOptions::default()),
                transform,
            ));
    }
}
