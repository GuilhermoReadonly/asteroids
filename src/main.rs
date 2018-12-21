mod ship;
mod drawable;
mod math;
mod constants;

#[macro_use]
extern crate log;

use std::process;
use std::thread;
use std::time::Duration;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::keyboard::Scancode;
use sdl2::render::WindowCanvas;

use crate::ship::Ship;
use crate::drawable::Drawable;
use crate::constants::{HEIGHT, WIDTH};

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
        thread::sleep(Duration::from_millis(100));
    }

    info!("Good bye my dude !");
    thread::sleep(Duration::from_millis(100));
}

fn treat_events(infernal_loop: &mut bool, event_pump: &mut EventPump, ship: &mut Ship){
    for event in event_pump.poll_iter() {
        debug!("Event : {:#?}", event);
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {// time to say good bye
                *infernal_loop = false;
            },
            _ => {}
        }
    }

    for pressed in event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode){
        info!("Movement : {:#?}", pressed);
        match pressed {
            Keycode::Z => {//up
                ship.move_up();
            },
            Keycode::S => {//down
                ship.move_down();
            },
            Keycode::Q => {//Left
                ship.move_left();
            },
            Keycode::D => {//Right
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
    let window = match video_subsystem.window("asteroids", WIDTH as u32, HEIGHT as u32).position_centered().build(){
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
