use amethyst::{
    core::{
        Transform,
    },
    ecs::{Join, System, WriteStorage},
};

// You'll have to mark as public in pong.rs
use crate::asteroids::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::components::{SpeedComponent};


pub struct SpeedSystem;
impl<'s> System<'s> for SpeedSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    WriteStorage<'s, SpeedComponent>
  );

  fn run(&mut self, (mut transforms, mut moveables): Self::SystemData) {
    for (moveable, transform) in (&mut moveables, &mut transforms).join() {
        //move the moveable thing on the other side of the box if it reaches one side
        if transform.translation().x > ARENA_WIDTH {
            transform.set_translation_x(0.0);
        }
        if transform.translation().x < 0.0 {
            transform.set_translation_x(ARENA_WIDTH);
        }
        if transform.translation().y > ARENA_HEIGHT {
            transform.set_translation_y(0.0);
        }
        if transform.translation().y < 0.0 {
            transform.set_translation_y(ARENA_HEIGHT);
        }

        //move the moveable thing proportionnaly to its speed
        transform.prepend_translation(moveable.speed);
    }
  }
}
