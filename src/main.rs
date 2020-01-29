#[macro_use]
extern crate log;

mod asteroids;
mod components;
mod systems;

use amethyst::{
    core::{
        transform::TransformBundle,
        bundle::SystemBundle
    },
    input::{InputBundle,StringBindings},
    prelude::*,
    renderer::{
        pass::DrawFlat,
        rendy::{
            graph::render::Pipeline,
            mesh::PosNormTex,
            hal::pso::Stage,
        },
    },
    utils::application_root_dir,
    window::DisplayConfig,
};

use self::asteroids::Asteroids;

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
        .start();

    info!("Let's the party rock !");

    let path = format!("{}/resources/display_config.ron", application_root_dir()?.to_str().unwrap());
    let config = DisplayConfig::load(&path);

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir()?.to_str().unwrap());
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    // let pipe = Pipeline::build().with_stage(
    //     Stage::with_backbuffer()
    //         .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
    //         .with_pass(DrawFlat::<PosNormTex>::new()),
    // );

    let game_data = GameDataBuilder::default()
        //.with_bundle(SystemBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::ShipSystem, "ship_system", &["input_system"])
        .with(systems::SpeedSystem, "speed_system", &["input_system"])
        .with(systems::ShipFireSystem,"ship_fire_system",&["input_system"]);

    let mut game = Application::new("./", Asteroids, game_data)?;
    game.run();

    info!("Bye my dude !");

    Ok(())
}
