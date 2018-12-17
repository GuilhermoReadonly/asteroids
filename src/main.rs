#[macro_use]
extern crate log;

use std::process;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub fn main() {

    let sdl_context = match sdl2::init() {
        Ok(f) => f,
        Err(error) => {
            error!("An error occured while getting the sdl context: {:?}", error);
            process::exit(1);
        },
    };

    let video_subsystem = match sdl_context.video() {
        Ok(f) => f,
        Err(error) => {
            error!("An error occured while getting the video subsystem: {:?}", error);
            process::exit(1);
        },
    };

    let window = match video_subsystem
    .window("asteroids", 800, 600)
    .position_centered()
    .build() {
        Ok(f) => f,
        Err(error) => {
            error!("An error occured while building the window: {:?}", error);
            process::exit(1);
        },
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(f) => f,
        Err(error) => {
            error!("An error occured while building the canvas: {:?}", error);
            process::exit(1);
        },
    };

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = match sdl_context.event_pump() {
        Ok(f) => f,
        Err(error) => {
            error!("An error occured while getting the event pump: {:?}", error);
            process::exit(1);
        },
    };
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        // The rest of the game loop goes here...

        canvas.present();
    }
}
