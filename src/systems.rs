use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{
    components::{Size, *},
    constants::*,
    entities::{BulletEntity, RockEntity},
    resources::*,
};

pub fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Velocity, &mut Transform)>,
) {
    for (_player, mut velocity, mut transform) in query.iter_mut() {
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
    mut query: Query<(&Player, &Transform, &mut TimeToFire)>,
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

pub fn life_system(
    mut game: ResMut<Game>,
    mut query: Query<(Entity, &Life, Option<&Rock>)>,
    mut commands: Commands,
) {
    for (entity, life, maybe_rock) in query.iter_mut() {
        if life.0 <= 0.0 {
            if maybe_rock.is_some() {
                game.rocks_destroyed += 1;
                info!("Total rock destroyed: {}", game.rocks_destroyed);
            }

            commands.entity(entity).despawn();
        }
    }
}

pub fn collision_player_rock_system(
    mut life_query: Query<
        &mut Life,
        (
            With<Collideable>,
            Or<(With<Player>, With<Rock>)>,
            With<Transform>,
            With<Size>,
        ),
    >,
    player_query: Query<(Entity, &Collideable, &Player, &Transform, &Size), With<Life>>,
    rock_query: Query<(Entity, &Collideable, &Rock, &Transform, &Size), With<Life>>,
) {
    for (player_entity, _, _, player_transform, player_size) in player_query.iter() {
        for (rock_entity, _, _, rock_transform, rock_size) in rock_query.iter() {
            let player_position = &player_transform.translation;
            let rock_position = &rock_transform.translation;

            if has_collided(player_position, player_size, rock_position, rock_size) {
                info!("Player has collided with rock ");
                {
                    let mut player_life: Mut<Life> = life_query
                        .get_mut(player_entity)
                        .expect("Entity does not exists !");
                    player_life.0 -= ROCK_COLLISION_DAMAGE;
                    info!("Player life is now {}", player_life.0);
                }

                {
                    let mut rock_life: Mut<Life> = life_query
                        .get_mut(rock_entity)
                        .expect("Entity does not exists !");
                    rock_life.0 -= ROCK_COLLISION_DAMAGE;

                    info!("Rock life is now {}", rock_life.0);
                }
            }
        }
    }
}

pub fn collision_bullet_rock_system(
    mut commands: Commands,
    mut life_query: Query<
        &mut Life,
        (
            With<Collideable>,
            Or<(With<Player>, With<Rock>)>,
            With<Transform>,
            With<Size>,
        ),
    >,
    bullet_query: Query<(Entity, &Collideable, &Bullet, &Transform, &Size)>,
    rock_query: Query<(Entity, &Collideable, &Rock, &Transform, &Size), With<Life>>,
) {
    for (bullet_entity, _, _, bullet_transform, bullet_size) in bullet_query.iter() {
        for (rock_entity, _, _, rock_transform, rock_size) in rock_query.iter() {
            let bullet_position = &bullet_transform.translation;
            let rock_position = &rock_transform.translation;

            if has_collided(bullet_position, bullet_size, rock_position, rock_size) {
                info!("Bullet has collided with rock ");
                {
                    commands.entity(bullet_entity).despawn();
                }

                {
                    let mut rock_life: Mut<Life> = life_query
                        .get_mut(rock_entity)
                        .expect("Entity does not exists !");
                    rock_life.0 -= BULLET_DAMAGE;

                    info!("Rock life is now {}", rock_life.0);
                }
            }
        }
    }
}

fn has_collided(position1: &Vec3, size1: &Size, position2: &Vec3, size2: &Size) -> bool {
    position1.x + size1.x / 2.0 > position2.x - size2.x / 2.0 && // p
    position1.x - size1.x / 2.0 < position2.x + size2.x / 2.0 && // l
    position1.y + size1.y / 2.0 > position2.y - size2.y / 2.0 && // o
    position1.y - size1.y / 2.0 < position2.y + size2.y / 2.0    // p
}

pub fn new_stage_system(mut game: ResMut<Game>, query: Query<(&Rock,)>, mut commands: Commands) {
    if query.iter().count() == 0 {
        game.stage += 1;

        for _ in 0..game.stage {
            RockEntity::new().spawn_rock(&mut commands);
        }
    }
}
