use std::error::Error;

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use components::*;
use constants::*;
use entities::*;
use resources::*;
use systems::*;

mod components;
#[allow(dead_code)]
mod constants;
#[allow(unused)]
mod entities;
mod resources;
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
        app.init_resource::<Game>()
            .add_startup_system(setup.system())
            .add_system(movement_system.system())
            .add_system(firing_system.system())
            .add_system(velocity_system.system())
            .add_system(angular_velocity_system.system())
            .add_system(time_to_live_system.system())
            .add_system(time_to_fire_system.system())
            .add_system(offscreen_system.system())
            .add_system(life_system.system())
            .add_system(collision_player_rock_system.system())
            .add_system(collision_bullet_rock_system.system())
            .add_system(new_stage_system.system());
    }
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    info!("Setup all we need to play...");

    game.rocks_destroyed = 0;
    game.stage = 1;
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // ship
    ShipEntity::new(Velocity(Vec3::new(0.0, 0.0, 0.0))).spawn_ship(&mut commands);

    // 1st rock
    RockEntity::new().spawn_rock(&mut commands);

    info!("Setup ready !!!");
}
