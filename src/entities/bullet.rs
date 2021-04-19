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
pub struct BulletEntity {
    bullet: Bullet,
    velocity: Velocity,
    time_to_live: TimeToLive,
    collideable: Collideable,
    size: Size,
}

impl<'a> BulletEntity {
    pub fn new() -> Self {
        Self {
            bullet: Bullet {},
            velocity: Velocity(BULLET_SPEED * Vec3::new(1.0, 0.0, 0.0)),
            time_to_live: TimeToLive(BULLET_TIME_TO_LIVE),
            collideable: Collideable,
            size: Size {
                x: BULLET_SIZE,
                y: BULLET_SIZE,
            },
        }
    }

    pub fn spawn_bullet(mut self, commands: &'a mut Commands, translation: Vec3, rotation: Quat) {
        let bullet = shapes::Circle {
            radius: BULLET_SIZE,
            center: Vec2::new(0.0, 0.0),
        };

        let transform = Transform {
            translation,
            rotation,
            ..Default::default()
        };

        self.velocity.0 = rotation.mul_vec3(self.velocity.0);

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
