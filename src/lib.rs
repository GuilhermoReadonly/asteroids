use std::error::Error;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use components::{Rotation, Ship, Velocity};
use constants::{SHIP_SIZE_X, SHIP_SIZE_Y};
use systems::{input_system, rotation_system, velocity_system};

mod components;
#[allow(dead_code)]
mod constants;
mod systems;

pub fn run() -> Result<(), Box<dyn Error>> {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(input_system.system())
        .add_system(velocity_system.system())
        .add_system(rotation_system.system())
        .run();

    Ok(())
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    info!("Setup all we need to play...");
    // Add the game's entities to our world

    let ship = shapes::Polygon {
        points: vec![
            Vec2::new(SHIP_SIZE_X, -SHIP_SIZE_Y),
            Vec2::new(0.0, SHIP_SIZE_Y),
            Vec2::new(-SHIP_SIZE_X, -SHIP_SIZE_Y),
            Vec2::new(0.0, 0.0),
        ],
        closed: true,
    };

    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        // ship
        .spawn(GeometryBuilder::build_as(
            &ship,
            materials.add(ColorMaterial::color(Color::GREEN)),
            TessellationMode::Stroke(StrokeOptions::default()),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ))
        .with(Ship {})
        .with(Velocity(Vec2::new(0.0, 0.0)))
        .with(Rotation(0.0));

    info!("Setup ready !!!");
}
