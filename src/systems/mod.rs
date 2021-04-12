use bevy::prelude::*;

use crate::{components::*, constants::*};

mod ship;
pub use ship::*;

pub fn velocity_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        *translation += time.delta_seconds() * velocity.0;
    }
}

pub fn angular_velocity_system(
    time: Res<Time>,
    mut query: Query<(&AngularVelocity, &mut Transform)>,
) {
    for (angular_velocity, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(
            time.delta_seconds() * angular_velocity.0,
        ));
    }
}

pub fn offscreen_system(mut query: Query<(&mut Transform,)>) {
    for (mut transform,) in query.iter_mut() {
        let mut pos = transform.translation;

        if pos.x > GAME_MAX_WIDTH || pos.x < -GAME_MAX_WIDTH {
            pos.x = pos.x - 2.0 * GAME_MAX_WIDTH * pos.x.signum()
        }
        if pos.y > GAME_MAX_HEIGHT || pos.y < -GAME_MAX_HEIGHT {
            pos.y = pos.y - 2.0 * GAME_MAX_HEIGHT * pos.y.signum()
        }
        transform.translation = pos;
    }
}

pub fn time_to_live_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
    mut commands: Commands,
) {
    for (entity, mut time_to_live) in query.iter_mut() {
        time_to_live.0 = time_to_live.0 - time.delta_seconds();

        if time_to_live.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn time_to_fire_system(time: Res<Time>, mut query: Query<(&mut TimeToFire,)>) {
    for (mut time_to_fire,) in query.iter_mut() {
        if time_to_fire.0 > 0.0 {
            time_to_fire.0 = time_to_fire.0 - time.delta_seconds();
        }
    }
}
