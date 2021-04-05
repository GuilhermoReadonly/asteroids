use std::error::Error;

use log::{info, LevelFilter};

use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    Config,
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

    asteroids_lib::run()
}
