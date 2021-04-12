use std::{error::Error, f32::consts::TAU};

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use components::{Rotation, Velocity};
use constants::*;
use entities::ShipEntity;
use systems::*;

mod components;
#[allow(dead_code)]
mod constants;
#[allow(unused)]
mod entities;
mod systems;

pub fn run() -> Result<(), Box<dyn Error>> {
    info!("Let the party rocks !");

    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            width: GAME_WINDOW_WIDTH,
            height: GAME_WINDOW_HEIGHT,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugin(AsteroidsPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .run();

    info!("It was freaking epic my dude, see ya around !");

    Ok(())
}

pub struct AsteroidsPlugin;
impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(movement_system.system())
            .add_system(firing_system.system())
            .add_system(velocity_system.system())
            .add_system(rotation_system.system())
            .add_system(time_to_live_system.system())
            .add_system(time_to_fire_system.system());
    }
}

fn setup(mut commands: Commands) {
    info!("Setup all we need to play...");
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // ship
    ShipEntity::new(Velocity(Vec2::new(0.0, 0.0)), Rotation(TAU / 4.0)).spawn_ship(&mut commands);

    info!("Setup ready !!!");
}
