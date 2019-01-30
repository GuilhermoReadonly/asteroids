use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark as public in pong.rs
use crate::asteroids::{Ship, ARENA_HEIGHT, ARENA_WIDTH};

pub struct ShipSystem;

const AMOUNT_TRANSLATION_FACTOR: f32 = 0.8;
const AMOUNT_ROTATION_FACTOR: f32 = 0.1;
const AMOUNT_SPEED_FACTOR: f32 = 0.05;
const MAX_SPEED: f32 = 1.0;
const MIN_SPEED: f32 = 0.0;

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

        ship.speed = (ship.speed + compute_movement(movement_y, AMOUNT_SPEED_FACTOR)).max(MIN_SPEED).min(MAX_SPEED);
        transform.move_up(ship.speed * AMOUNT_TRANSLATION_FACTOR);
        transform.roll_local(compute_movement(movement_x, AMOUNT_ROTATION_FACTOR));
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
