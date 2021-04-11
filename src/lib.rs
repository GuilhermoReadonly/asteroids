use std::error::Error;

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use components::{Rotation, Velocity};
use entities::ShipEntity;
use systems::{input_system, rotation_system, velocity_system};

mod components;
#[allow(dead_code)]
mod constants;
#[allow(unused)]
mod entities;
mod systems;

pub fn run() -> Result<(), Box<dyn Error>> {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(input_system.system())
        .add_system(velocity_system.system())
        .add_system(rotation_system.system())
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    info!("Setup all we need to play...");
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // ship
    ShipEntity::new(Velocity(Vec2::new(0.0, 0.0)), Rotation(0.0)).spawn_ship(&mut commands);

    info!("Setup ready !!!");
}
