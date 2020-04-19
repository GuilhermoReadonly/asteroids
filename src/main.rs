use log::{error, info};
use log4rs;

mod asteroid;
mod constants;
mod inputs;
mod objects;

use crate::{
    asteroid::AsteroidWorld,
    constants::{ARENA_HEIGHT, ARENA_WIDTH, GAME_AUTHOR, GAME_NAME},
};

use ggez::{
    conf::{NumSamples, WindowMode, WindowSetup},
    event, ContextBuilder,
};

fn main() {
    // Init logger
    log4rs::init_file("./resources/log4rs.yml", Default::default()).unwrap();

    // Make a Context and an EventLoop.
    let window_setup = WindowSetup::default()
        .title(GAME_NAME)
        .samples(NumSamples::Zero)
        .vsync(true);
    let window_mode = WindowMode::default().dimensions(ARENA_WIDTH, ARENA_HEIGHT);
    let (mut ctx, mut event_loop) = ContextBuilder::new(GAME_NAME, GAME_AUTHOR)
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut asteroid_game = AsteroidWorld::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut asteroid_game) {
        Ok(_) => info!("It was freaking epic my dude, see ya around !"),
        Err(e) => error!("Oh my man, the shit has hit the fan: {}", e),
    };
}
