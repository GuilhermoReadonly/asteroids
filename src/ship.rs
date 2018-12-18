use crate::drawable::Drawable;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::{Rect};

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Ship {
    pub position: Point,
}

impl Drawable for Ship {
    fn draw(&self, canvas: & mut WindowCanvas) -> () {
        info!("Draw : {:?}", self);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let rect = Rect::new(self.position.x as i32, self.position.y as i32, 10, 10);
        let result = canvas.draw_rect(rect);
        debug!("result : {:?}", result);
    }
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            position: Point {
                x: 0.0,
                y: 0.0,
            }
        }
    }

    pub fn move_up(& mut self) -> () {
        self.position.y += 1.0;
    }
    pub fn move_down(& mut self) -> () {
        self.position.y -= 1.0;
    }
    pub fn move_right(& mut self) -> () {
        self.position.x += 1.0;
    }
    pub fn move_left(& mut self) -> () {
        self.position.x -= 1.0;
    }
}
