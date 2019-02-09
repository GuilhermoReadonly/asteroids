mod asteroids;
mod systems;

use amethyst::{
        prelude::*,
        renderer::{
            DisplayConfig, DrawFlat, Pipeline,RenderBundle, Stage, PosNormTex,
        },
        utils::application_root_dir,
        core::transform::TransformBundle,
        input::InputBundle,
    };

use self::asteroids::Asteroids;

fn main() -> amethyst::Result<()> {

    amethyst::Logger::from_config(Default::default())
    .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
    .start();
    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let binding_path = format!("{}/resources/bindings_config.ron",application_root_dir());
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;


    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat::<PosNormTex>::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::ShipSystem, "ship_system", &["input_system"]);;

    let mut game = Application::new("./", Asteroids, game_data)?;
    game.run();

    Ok(())
}
