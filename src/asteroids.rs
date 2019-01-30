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
        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);
        initialise_ship(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

#[derive(Debug)]
pub struct Ship {
    pub speed: f32,
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

fn initialise_ship(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // Assign the sprites for the ship
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // ship is the first sprite in the sprite_sheet
    };

    let mut ship_transform = Transform::default();

    // Correctly position the ship in the middle of the arena.
    let y = ARENA_HEIGHT / 2.0;
    let x = ARENA_WIDTH / 2.0;
    ship_transform
        .set_xyz(x, y, 0.0)
        .set_scale(0.2,0.2,0.2);

    // Create a ship entity.
    world
        .create_entity()
        .with(sprite_render)
        .with(Ship::new())
        .with(ship_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/ship_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/ship_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}
