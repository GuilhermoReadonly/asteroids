use crate::drawable::Drawable;
use crate::constants::{HEIGHT, WIDTH};
use crate::math::rotate;

use std::f64::consts::PI;

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;


pub const DIMENSION: i32 = 10;

#[derive(Debug)]
pub struct Ship {
    pub position: Point,
    pub direction: f64,
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
}

impl Drawable for Ship {
    fn draw(&self, canvas: &mut WindowCanvas) -> () {
        info!("Draw : {:?}", self);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line(self.point1, self.point2).unwrap();
        canvas.draw_line(self.point1, self.point3).unwrap();
        canvas.draw_line(self.point2, self.point3).unwrap();
    }
}

impl Ship {
    pub fn new() -> Ship {
        let xc = WIDTH/2 as i32;
        let yc = HEIGHT/2 as i32;
        Ship {
            position: Point::new(xc , yc),
            direction: 0.0,
            point1: Point::new(xc - DIMENSION ,yc + DIMENSION),
            point2: Point::new(xc + DIMENSION , yc + DIMENSION),
            point3: Point::new(xc , yc - DIMENSION),
        }
    }

    pub fn move_up(&mut self) -> () {
        self.position.y -= 1;
        self.compute_movements();
    }
    pub fn move_down(&mut self) -> () {
        self.position.y += 1;
        self.compute_movements();
    }
    pub fn move_right(&mut self) -> () {
        self.direction += 0.1; //PI/32.0;
        self.compute_movements();
    }
    pub fn move_left(&mut self) -> () {
        self.direction -= 0.1; //PI/32.0;
        self.compute_movements();
    }

    pub fn rotate_all(&mut self) -> () {
        self.point1 = rotate(&self.point1, &self.position, &self.direction);
        self.point2 = rotate(&self.point2, &self.position, &self.direction);
        self.point3 = rotate(&self.point3, &self.position, &self.direction);
    }

    pub fn translate_all(&mut self) -> () {
        self.point1 = Point::new(self.position.x as i32 - DIMENSION , self.position.y as i32 + DIMENSION);
        self.point2 = Point::new(self.position.x as i32 + DIMENSION , self.position.y as i32 + DIMENSION);
        self.point3 = Point::new(self.position.x as i32 , self.position.y as i32 - DIMENSION);
    }

    pub fn compute_movements(&mut self) -> () {
        self.translate_all();
        self.rotate_all();
    }
}
