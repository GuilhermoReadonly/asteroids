use amethyst::{
    core::{
        Transform,
        nalgebra::base::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage},
    input::InputHandler
};

// You'll have to mark as public in pong.rs
use crate::asteroids::{Ship, ARENA_HEIGHT, ARENA_WIDTH};

pub struct ShipSystem;

const AMOUNT_ROTATION_FACTOR: f32 = 0.1;
const AMOUNT_SPEED_FACTOR: f32 = 0.01;

impl<'s> System<'s> for ShipSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Ship>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, mut ships, input): Self::SystemData) {
    for (ship, transform) in (&mut ships, &mut transforms).join() {
        let movement_x = input.axis_value("ship_x");
        let movement_y = input.axis_value("ship_y");

        //compute rotation
        if movement_x != Some(0.0) {
            transform.roll_local(compute_movement(movement_x, AMOUNT_ROTATION_FACTOR));
        }

        //compute translation
        if movement_y != Some(0.0) {
            let delta_speed_vector = Vector3::y() * compute_movement(movement_y, AMOUNT_SPEED_FACTOR);
            ship.speed = ship.speed + (transform.isometry() * delta_speed_vector);
        }

        //move the ship on the other side of the box if it reaches one side
        if transform.translation().x > ARENA_WIDTH {
            transform.set_x(0.0);
        }
        if transform.translation().x < 0.0 {
            transform.set_x(ARENA_WIDTH);
        }
        if transform.translation().y > ARENA_HEIGHT {
            transform.set_y(0.0);
        }
        if transform.translation().y < 0.0 {
            transform.set_y(ARENA_HEIGHT);
        }
        transform.move_global(ship.speed);
    }
  }
}

fn compute_movement(movement: Option<f64>, movement_factor: f32) -> f32{
    let mut computed_move = 0.0;
    if let Some(mv_amount) = movement {
        if mv_amount != 0.0 {
            // TODO: use amethyst::core::timing::Time to get the framerate and compute the
            // distance to move
            computed_move = movement_factor * mv_amount as f32;
        }
    }
    computed_move
}
