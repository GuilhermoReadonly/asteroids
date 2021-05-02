use std::error::Error;

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use constants::*;
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

    let mut app = App::build();
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            width: GAME_WINDOW_WIDTH,
            height: GAME_WINDOW_HEIGHT,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AsteroidsPlugin)
        .add_plugin(ShapePlugin);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();

    info!("It was freaking epic my dude, see ya around !");

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    InGame,
    GameOver,
}

pub struct ColorMaterials {
    pub black: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
}

impl FromWorld for ColorMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ColorMaterials {
            black: materials.add(BLACK.into()),
            green: materials.add(GREEN.into()),
        }
    }
}

pub struct AsteroidsPlugin;
impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ColorMaterials>()
            .init_resource::<Game>()
            .add_system(velocity_system.system())
            .add_system(angular_velocity_system.system())
            .add_system(offscreen_system.system())
            .add_state(AppState::Menu)
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu_system.system()))
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game.system()))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(movement_system.system())
                    .with_system(firing_system.system())
                    .with_system(time_to_live_system.system())
                    .with_system(time_to_fire_system.system())
                    .with_system(life_system.system())
                    .with_system(collision_player_rock_system.system())
                    .with_system(collision_bullet_rock_system.system())
                    .with_system(new_stage_system.system())
                    .with_system(game_over_system.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::GameOver).with_system(setup_game_over.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::GameOver)
                    .with_system(return_to_menu_system.system()),
            );
    }
}
