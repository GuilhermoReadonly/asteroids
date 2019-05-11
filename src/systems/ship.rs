use amethyst::{
    core::{nalgebra::base::Vector3, Transform},
    ecs::{Join, Read, System, WriteStorage},
    input::InputHandler,
};

// You'll have to mark as public in pong.rs
use crate::components::ShipComponent;
use crate::components::SpeedComponent;

const AMOUNT_ROTATION_FACTOR: f32 = 0.1;
const AMOUNT_SPEED_FACTOR: f32 = 0.01;

pub struct ShipSystem;
impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ShipComponent>,
        WriteStorage<'s, SpeedComponent>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, mut ships, mut speeds, input): Self::SystemData) {
        for (_ship, speed, transform) in (&mut ships,&mut speeds, &mut transforms).join() {
            let movement_x = input.axis_value("ship_x");
            let movement_y = input.axis_value("ship_y");

            //compute rotation
            if movement_x != Some(0.0) {
                transform.roll_local(compute_movement(movement_x, AMOUNT_ROTATION_FACTOR));
            }

            //compute translation
            if movement_y != Some(0.0) {
                let delta_speed_vector =
                    Vector3::y() * compute_movement(movement_y, AMOUNT_SPEED_FACTOR);
                speed.speed = speed.speed + (transform.isometry() * delta_speed_vector);
            }
        }
    }
}

fn compute_movement(movement: Option<f64>, movement_factor: f32) -> f32 {
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

pub struct ShipFireSystem;
impl<'s> System<'s> for ShipFireSystem {
    type SystemData = Read<'s, InputHandler<String, String>>;

    fn run(&mut self, input: Self::SystemData) {
        //compute firing
        if Some(true) == input.action_is_down("shoot") {
            debug!("SHOOT THE MOTHAFUCKAAAAAAAAAA !!!!!!!");
            // let mut bullet_transform = Transform::default();
            // bullet_transform
            // .set_xyz(transform.translation().x, transform.translation().y, 0.0)
            // .set_scale(1.0,1.0,1.0);
            //
            // // Create a bullet entity.
            // entities
            // .build_entity()
            // .with(assets.simple_bullet.clone(), &mut mesh_handles)
            // .with(assets.color.clone(), &mut materials)
            // .with(SimpleBulletComponent::new(ship.speed.clone()), &mut bullets)
            // .with(bullet_transform, &mut transforms)
            // .build();
        }
    }
}
