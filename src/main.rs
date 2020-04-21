use log::{error, info, LevelFilter};
use log4rs;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
};
use std::error::Error;

mod asteroid;
mod constants;
mod inputs;
mod objects;

use crate::{
    asteroid::AsteroidWorld,
    constants::{GAME_AUTHOR, GAME_NAME, GAME_WINDOW_HEIGHT, GAME_WINDOW_WIDTH},
};

use ggez::{
    conf::{NumSamples, WindowMode, WindowSetup},
    event, ContextBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Init logger
    match log4rs::init_file("./resources/log4rs.yml", Default::default()) {
        Err(err) => {
            println!("For a reason, the little shit called a logger didn't init its lazy-ass from file !!! {}",err);
            let stdout = ConsoleAppender::builder().build();
            let root = Root::builder().appender("stdout").build(LevelFilter::Info);
            let config = Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .build(root)
                .unwrap();
            match log4rs::init_config(config) {
                Err(err) => {
                    println!(
                        "The lazy-ass little shit didn't even bother init from code !!! {}",
                        err
                    );
                }
                _ => {
                    info!("But the logger successfully init its shit from code...");
                }
            };
        }
        _ => {
            info!("The logger successfully init its shit...");
        }
    };

    // Make a Context and an EventLoop.
    let window_setup = WindowSetup::default()
        .title(GAME_NAME)
        .samples(NumSamples::Zero)
        .vsync(true);
    let window_mode = WindowMode::default().dimensions(GAME_WINDOW_WIDTH, GAME_WINDOW_HEIGHT);
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
    Ok(())
}
