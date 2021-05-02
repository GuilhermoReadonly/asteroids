use bevy::prelude::*;

use crate::{components::*, constants::*};

mod in_game;
pub use in_game::*;

mod menu;
pub use menu::*;

mod game_over;
pub use game_over::*;

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
