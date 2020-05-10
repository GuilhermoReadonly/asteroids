use asteroids;
use log::{error, info, LevelFilter};
use log4rs;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
};
use std::error::Error;

use crate::asteroids::{constants::*, MainState};

use ggez::{
    conf::{NumSamples, WindowMode, WindowSetup},
    event, graphics, ContextBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Init logger
    log4rs::init_file("./resources/log4rs.yml", Default::default())
        .and_then(|_| Ok(info!("The logger successfully init its stuffs...")))
        .or_else(|err| {
            println!("For a reason, the little shit called LoGgEr didn't init its lazy-ass from file!!! {}",err);
            let stdout = ConsoleAppender::builder().build();
            let root = Root::builder().appender("stdout").build(LevelFilter::Info);
            let config = Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .build(root)?;
            log4rs::init_config(config)?;
            Ok(info!("But the logger successfully init its shit from code..."))
        })
        .map_err(|err: Box<dyn Error>|{
            println!("(ノಠДಠ)ノ彡┻━┻ {}", err);
            err
        })?;

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
    let mut asteroid_game = MainState::new();

    let rect = graphics::Rect::new(
        -GAME_MAX_WIDTH,
        -GAME_MAX_HEIGHT,
        GAME_WINDOW_WIDTH,
        GAME_WINDOW_HEIGHT,
    );
    graphics::set_screen_coordinates(&mut ctx, rect)?;

    // Run!
    event::run(&mut ctx, &mut event_loop, &mut asteroid_game)
        .map(|_| info!("It was freaking epic my dude, see ya around !"))
        .map_err(|e| {
            error!("Oh my man, the shit has severly hit the fan: {}", e);
            e.into()
        })
}
