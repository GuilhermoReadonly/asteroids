use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark as public in pong.rs
use crate::asteroids::{Ship, ARENA_HEIGHT, ARENA_WIDTH};

pub struct ShipSystem;

const AMOUNT_FACTOR: f32 = 0.7;

impl<'s> System<'s> for ShipSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Ship>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, ships, input): Self::SystemData) {
    for (ship, transform) in (&ships, &mut transforms).join() {
        let movement_x = input.axis_value("ship_x");
        let movement_y = input.axis_value("ship_y");
        transform.set_x(
            (transform.translation().x + compute_movement(movement_x))
                .max(0.0)
                .min(ARENA_HEIGHT),
        );
        transform.set_y(
            (transform.translation().y + compute_movement(movement_y))
                .max(0.0)
                .min(ARENA_WIDTH),
        );
    }
  }
}

fn compute_movement(movement: Option<f64>) -> f32{
    let mut computed_move = 0.0;
    if let Some(mv_amount) = movement {
        if mv_amount != 0.0 {
            // TODO: use amethyst::core::timing::Time to get the framerate and compute the
            // distance to move accordingly instead of using the fixed constant AMOUNT_FACTOR
            computed_move = AMOUNT_FACTOR * mv_amount as f32;
        }
    }
    computed_move
}
