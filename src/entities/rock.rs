use std::f32::consts::TAU;

use bevy::{
    math::{Vec2, Vec3},
    prelude::*,
    prelude::{Assets, Color, Transform},
    sprite::ColorMaterial,
};
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

use crate::{components::*, constants::*};

#[derive(Bundle)]
pub struct RockEntity {
    velocity: Velocity,
    angular_velocity: AngularVelocity,
}

impl<'a> RockEntity {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let velocity = Velocity(Vec3::new(
            rng.gen_range(-ROCK_MAX_SPEED..ROCK_MAX_SPEED),
            rng.gen_range(-ROCK_MAX_SPEED..ROCK_MAX_SPEED),
            0.,
        ));
        let angular_velocity =
            AngularVelocity(rng.gen_range(-ROCK_MAX_ANGLE_SPEED..ROCK_MAX_ANGLE_SPEED));
        Self {
            velocity,
            angular_velocity,
        }
    }

    pub fn spawn_rock(self, commands: &'a mut Commands) {
        let mut rng = rand::thread_rng();
        let mut points: Vec<Vec2> = vec![];
        let angle_between_edges = TAU / ROCK_NB_EDGES as f32;

        let position = Vec3::new(
            rng.gen_range(-GAME_MAX_WIDTH..GAME_MAX_WIDTH),
            rng.gen_range(-GAME_MAX_HEIGHT..GAME_MAX_HEIGHT),
            0.,
        );

        for i in 0..ROCK_NB_EDGES {
            let current_radius: f32 = rng.gen_range(
                ROCK_RADIUS_INIT - ROCK_RADIUS_DELTA..ROCK_RADIUS_INIT + ROCK_RADIUS_DELTA,
            );
            let x: f32 = (i as f32 * angle_between_edges).cos() * current_radius;
            let y: f32 = (i as f32 * angle_between_edges).sin() * current_radius;
            let point = Vec2::new(x, y);
            points.push(point);
        }

        let rock = shapes::Polygon {
            points,
            closed: true,
        };

        let transform = Transform {
            translation: position,
            ..Default::default()
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &rock,
                ShapeColors::new(Color::GREEN),
                DrawMode::Stroke(StrokeOptions::default()),
                transform,
            ))
            .insert_bundle(self);
    }
}
