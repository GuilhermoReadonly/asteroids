#[macro_use]
extern crate log;

use std::process;
use std::thread;
use std::time::Duration;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use crate::ship::Ship;
use crate::drawable::Drawable;

mod ship;
mod drawable;

pub fn main() {

    if let Err(e) = log4rs::init_file("log4rs.yml", Default::default()) {
        println!("Error while reading log4rs.yml: {}", e);
        process::exit(1);
    }

    let (mut canvas,sdl_context) = match init_sdl() {
        Ok(f) => f,
        Err(error) => {
            error!("An error occured while initilizing sdl: {:?}", error);
            process::exit(1);
        },
    };

    info!("init sdl Ok.");

    let mut event_pump = match sdl_context.event_pump() {
        Ok(f) => {
            info!("retrieve event pump Ok."); f
        },
        Err(error) => {
            error!("An error occured while getting the event pump: {:?}", error);
            process::exit(1);
        },
    };

    let mut ship = Ship::new();

    info!("Let's start the infernal loop !");
    let mut infernal_loop = true;
    while infernal_loop {
        treat_events(&mut infernal_loop, &mut event_pump, &mut ship);
        draw_all(&mut canvas, &mut ship);
    }

    info!("Good bye my dude !");
    thread::sleep(Duration::from_millis(100));
}

fn treat_events(infernal_loop: &mut bool, event_pump: &mut EventPump, ship: &mut Ship){
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {// time to say good bye
                *infernal_loop = false;
            },
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => {//up
                ship.move_up();
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {//down
                ship.move_down();
            },
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => {//Left
                ship.move_left();
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {//Right
                ship.move_right();
            },
            _ => {}
        }
    }
}

fn draw_all(canvas: &mut WindowCanvas, ship: &mut Ship){
    canvas.clear();
    draw_background(canvas);
    ship.draw(canvas);
    canvas.present();
}

fn draw_background(canvas: & mut WindowCanvas){
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let result = canvas.fill_rect(Option::None);
    debug!("result : {:?}", result);
}


pub fn init_sdl() -> Result<(WindowCanvas, Sdl), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = match video_subsystem.window("asteroids", 800, 600).position_centered().build(){
        Ok(f) => Ok(f),
        Err(error) => {
            Err(format!("An error occured while building the window: {:?}", error))
        },
    }?;

    match window.into_canvas().present_vsync() //< this means the screen cannot
    // render faster than your display rate (usually 60Hz or 144Hz)
    .build(){
        Ok(window_canvas) => Ok((window_canvas, sdl_context)),
        Err(error) => {
            Err(format!("An error occured while building the canvas: {:?}", error))
        },
    }
}
