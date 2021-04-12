use bevy::prelude::*;

use crate::{components::*, constants::*, entities::BulletEntity};

pub fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ship, &mut Velocity, &mut Rotation)>,
) {
    for (_ship, mut velocity, mut rotation) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            rotation.0 += std::f32::consts::TAU * 0.01;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            rotation.0 -= std::f32::consts::TAU * 0.01;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            velocity.0 += Vec2::new(rotation.0.cos(), rotation.0.sin());
        }
        if keyboard_input.pressed(KeyCode::Down) {
            velocity.0 -= Vec2::new(rotation.0.cos(), rotation.0.sin());
        }
    }
}

pub fn firing_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ship, &Rotation, &Transform, &mut TimeToFire)>,
    mut commands: Commands,
) {
    for (_ship, rotation, transform, mut time_to_fire) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) && time_to_fire.0 <= 0.0 {
            debug!("shoot !");
            let position = transform.translation;
            BulletEntity::new(rotation.clone()).spawn_bullet(&mut commands, position);

            time_to_fire.0 = SHIP_RELOAD_TIME;
        }
    }
}
