use bevy::prelude::*;

use crate::components::{Rotation, Ship, Velocity};

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ship, &mut Velocity, &mut Rotation)>,
) {
    for (_ship, mut velocity, mut rotation) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            rotation.0 += std::f32::consts::TAU * 0.001;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            rotation.0 -= std::f32::consts::TAU * 0.001;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.0 = velocity.0 + Vec2::normalize(velocity.0.clone()) * 0.001;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            velocity.0 = velocity.0 - Vec2::normalize(velocity.0.clone()) * 0.001;
        }
    }
}

pub fn velocity_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        *translation += (time.delta_seconds() * velocity.0).extend(0.0);
    }
}

pub fn rotation_system(mut query: Query<(&Rotation, &mut Transform)>) {
    for (rotation, mut transform) in query.iter_mut() {
        let transform_rotation = &mut transform.rotation;

        *transform_rotation = Quat::from_axis_angle(Vec3::unit_z(), rotation.0);
    }
}
