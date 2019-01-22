use amethyst::{
        prelude::*,
        assets::{AssetStorage, Loader},
        core::transform::Transform,
        ecs::prelude::{Component, DenseVecStorage},
        renderer::{
            Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
            SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
        }
    };

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct Asteroids;

impl SimpleState for Asteroids {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Ship>();
        initialise_ship(world);
        initialise_camera(world);
    }
}


pub struct Ship {
    pub speed: f64,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            speed: 0.0,
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
    let mut ship_transform = Transform::default();

    // Correctly position the ship in the middle of the arena.
    let y = ARENA_HEIGHT / 2.0;
    let x = ARENA_WIDTH / 2.0;
    ship_transform.set_xyz(x, y, 0.0);

    // Create a ship entity.
    world
        .create_entity()
        .with(Ship::new())
        .with(ship_transform)
        .build();
}
