use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::components::{Ball, Collider, Paddle, Scoreboard};

pub fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction * paddle.speed;
        // bound the paddle within the walls
        translation.x = translation.x.min(380.0).max(-380.0);
    }
}

pub fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    for (ball, mut transform) in ball_query.iter_mut() {
        transform.translation += ball.velocity * delta_seconds;
    }
}

pub fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}

pub fn ball_collision_system(
    commands: &mut Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    for (mut ball, ball_transform, sprite) in ball_query.iter_mut() {
        let ball_size = sprite.size;
        let velocity = &mut ball.velocity;

        // check collision with walls
        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::Scorable = *collider {
                    scoreboard.score += 1;
                    commands.despawn(collider_entity);
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the collision
                match collision {
                    Collision::Left => reflect_x = velocity.x > 0.0,
                    Collision::Right => reflect_x = velocity.x < 0.0,
                    Collision::Top => reflect_y = velocity.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.y > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    velocity.x = -velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    velocity.y = -velocity.y;
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is also in collision
                if let Collider::Solid = *collider {
                    break;
                }
            }
        }
    }
}
