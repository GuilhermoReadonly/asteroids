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
        Ship {
            position: Point::new(WIDTH/2 as i32 , HEIGHT/2 as i32),
            direction: 0.0,
            point1: Point::new(HEIGHT/2 as i32 - DIMENSION , WIDTH/2 as i32 + DIMENSION),
            point2: Point::new(HEIGHT/2 as i32 + DIMENSION , WIDTH/2 as i32 + DIMENSION),
            point3: Point::new(HEIGHT/2 as i32 , WIDTH/2 as i32 - DIMENSION),
        }
    }

    pub fn move_up(&mut self) -> () {
        self.position.y -= 1;

        self.point1 = Point::new(self.position.x as i32 - DIMENSION , self.position.y as i32 + DIMENSION);
        self.point2 = Point::new(self.position.x as i32 + DIMENSION , self.position.y as i32 + DIMENSION);
        self.point3 = Point::new(self.position.x as i32 , self.position.y as i32 - DIMENSION);
    }
    pub fn move_down(&mut self) -> () {
        self.position.y += 1;

        self.point1 = Point::new(self.position.x as i32 - DIMENSION , self.position.y as i32 + DIMENSION);
        self.point2 = Point::new(self.position.x as i32 + DIMENSION , self.position.y as i32 + DIMENSION);
        self.point3 = Point::new(self.position.x as i32 , self.position.y as i32 - DIMENSION);
    }
    pub fn move_right(&mut self) -> () {
        self.direction -= 0.1 % (2.0 * PI);

        self.point1 = rotate(&self.point1, &self.position, &-0.1);
        self.point2 = rotate(&self.point2, &self.position, &-0.1);
        self.point3 = rotate(&self.point3, &self.position, &-0.1);
    }
    pub fn move_left(&mut self) -> () {
        self.direction += 0.1 % (2.0 * PI);

        self.point1 = rotate(&self.point1, &self.position, &0.1);
        self.point2 = rotate(&self.point2, &self.position, &0.1);
        self.point3 = rotate(&self.point3, &self.position, &0.1);
    }
}
