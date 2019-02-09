use amethyst::{
    prelude::*,
    assets::{Loader, ProgressCounter},
    core::{
        transform::Transform,
        nalgebra::base::Vector3,
    },
    ecs::prelude::{Component, DenseVecStorage},
    renderer::{
        Camera, Projection, Material, MeshHandle, ObjFormat, MaterialDefaults,
    },
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct Asteroids;

impl SimpleState for Asteroids {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // Load the things necessary to render the graphics.
        initialise_ship(world);
        initialise_camera(world);
    }
}

#[derive(Debug)]
pub struct Ship {
    pub speed: Vector3<f32>,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            speed: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
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

fn initialise_ship(world: &mut World) {
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

        Assets {
            ship,
            color,
        }
    };

    //these 2 lines are not necessary but will be useful when we'll add a loading stage
    world.add_resource(assets);
    let assets = world.read_resource::<Assets>().clone();

    let mut ship_transform = Transform::default();

    // Correctly position the ship in the middle of the arena.
    let y = ARENA_HEIGHT / 2.0;
    let x = ARENA_WIDTH / 2.0;
    ship_transform
        .set_xyz(x, y, 0.0)
        .set_scale(1.0,1.0,1.0);

    // Create a ship entity.
    world
        .create_entity()
        .with(assets.ship.clone())
        .with(assets.color.clone())
        .with(Ship::new())
        .with(ship_transform)
        .build();
}

#[derive(Clone)]
pub struct Assets {
    ship: MeshHandle,
    color: Material,
}
