use crate::drawable::Drawable;
use crate::constants::{HEIGHT, WIDTH};
use crate::math::rotate;
use crate::math::translate;

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;


pub const DIMENSION: i32 = 10;

#[derive(Debug, PartialEq)]
pub struct Ship {
    pub position: PointExact,
    pub angle: f64,
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
}

#[derive(Debug, PartialEq)]
pub struct PointExact {
    pub x: f64,
    pub y: f64,
}

impl Drawable for Ship {
    fn draw(&self, canvas: &mut WindowCanvas) -> () {
        debug!("Draw : {:#?}", self);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line(self.point1, self.point2).unwrap();
        canvas.draw_line(self.point1, self.point3).unwrap();
        canvas.draw_line(self.point2, self.point3).unwrap();
    }
}

impl Ship {
    pub fn new() -> Ship {
        let xc = WIDTH/2;
        let yc = HEIGHT/2;
        Ship {
            position: PointExact{x: xc as f64 , y: yc as f64},
            angle: 0.0,
            point1: Point::new(xc - DIMENSION ,yc + DIMENSION),
            point2: Point::new(xc + DIMENSION , yc + DIMENSION),
            point3: Point::new(xc , yc - DIMENSION),
        }
    }

    pub fn move_up(&mut self) -> () {
        self.position = translate(&self.position, &3.0, &self.angle);
        self.compute_movements();
    }
    pub fn move_down(&mut self) -> () {
        self.position = translate(&self.position, &-1.0, &self.angle);
        self.compute_movements();
    }
    pub fn turn_right(&mut self) -> () {
        self.angle -= 0.1; //PI/32.0;
        self.compute_movements();
    }
    pub fn turn_left(&mut self) -> () {
        self.angle += 0.1; //PI/32.0;
        self.compute_movements();
    }

    fn rotate_all(&mut self) -> () {
        self.point1 = rotate(&self.point1, &self.position, &self.angle);
        self.point2 = rotate(&self.point2, &self.position, &self.angle);
        self.point3 = rotate(&self.point3, &self.position, &self.angle);
    }

    fn create_all(&mut self) -> () {
        self.point1 = Point::new(self.position.x.round() as i32 - DIMENSION , self.position.y.round()  as i32 + DIMENSION);
        self.point2 = Point::new(self.position.x.round()  as i32 + DIMENSION , self.position.y.round()  as i32 + DIMENSION);
        self.point3 = Point::new(self.position.x.round()  as i32 , self.position.y.round()  as i32 - DIMENSION);
    }

    fn compute_movements(&mut self) -> () {
        self.create_all();
        self.rotate_all();
    }
}
