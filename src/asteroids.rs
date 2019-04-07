use amethyst::{
    assets::{Loader, ProgressCounter},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, Material, MaterialDefaults, MeshHandle, ObjFormat, Projection},
};

use crate::components::ShipComponent;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct Asteroids;

impl SimpleState for Asteroids {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // Load the things necessary to render the graphics.
        initialise_assets(world);
        initialise_camera(world);
        ShipComponent::spawn_ship(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialise_assets(world: &mut World) {
    let mut progress = ProgressCounter::default();
    let assets = {
        let loader = world.read_resource::<Loader>();
        let tex_storage = world.read_resource();
        let mesh_storage = world.read_resource();
        let mat_defaults = world.read_resource::<MaterialDefaults>();

        let color = loader.load_from_data([0.0, 1.0, 0.0, 1.0].into(), &mut progress, &tex_storage);
        let color = Material {
            albedo: color,
            ..mat_defaults.0.clone()
        };
        let ship = loader.load(
            "resources/ship.obj",
            ObjFormat,
            (),
            &mut progress,
            &mesh_storage,
        );

        let simple_bullet = loader.load(
            "resources/simple_bullet.obj",
            ObjFormat,
            (),
            &mut progress,
            &mesh_storage,
        );

        Assets {
            ship,
            simple_bullet,
            color,
        }
    };

    world.add_resource(assets);
}

#[derive(Clone)]
pub struct Assets {
    pub ship: MeshHandle,
    pub simple_bullet: MeshHandle,
    pub color: Material,
}
