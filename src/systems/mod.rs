use bevy::prelude::*;

use crate::components::*;

mod ship;
pub use ship::*;

pub fn velocity_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        *translation += (time.delta_seconds() * velocity.0).extend(0.0);
    }
}

pub fn rotation_system(mut query: Query<(&Rotation, &mut Transform)>) {
    for (rotation, mut transform) in query.iter_mut() {
        let transform_rotation = &mut transform.rotation;

        *transform_rotation = Quat::from_axis_angle(Vec3::Z, rotation.0);
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
