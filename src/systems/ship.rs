use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{components::*, constants::*, entities::BulletEntity};

pub fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ship, &mut Velocity, &mut Transform)>,
) {
    for (_ship, mut velocity, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            debug!("Left");
            transform.rotate(Quat::from_rotation_z(TAU * 0.01));
        }
        if keyboard_input.pressed(KeyCode::Right) {
            debug!("Right");
            transform.rotate(Quat::from_rotation_z(-TAU * 0.01));
        }

        let direction = transform.rotation * Vec3::X * SHIP_THRUST;
        if keyboard_input.pressed(KeyCode::Up) {
            debug!("Up");
            velocity.0 += direction;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            debug!("Down");
            velocity.0 -= direction;
        }
    }
}

pub fn firing_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ship, &Transform, &mut TimeToFire)>,
    mut commands: Commands,
) {
    for (_ship, transform, mut time_to_fire) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            debug!("Space");
            if time_to_fire.0 <= 0.0 {
                debug!("shoot !");
                let position = transform.translation;
                //let (_,rotation) = transform.rotation.to_axis_angle();
                BulletEntity::new().spawn_bullet(&mut commands, position, transform.rotation);

                time_to_fire.0 = SHIP_RELOAD_TIME;
            }
        }
    }
}
