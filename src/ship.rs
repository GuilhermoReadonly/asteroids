use std::f64::consts::PI;

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::drawable::Drawable;
use crate::constants::{HEIGHT, WIDTH};
use crate::math::rotate;
use crate::math::translate;

pub const DIMENSION: i32 = 7;

#[derive(Debug, PartialEq)]
pub struct Ship {
    pub position: PointExact,
    pub angle: f64,
    pub points: Vec<PointWithOffset>,
}

#[derive(Debug, PartialEq)]
pub struct PointExact {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq)]
pub struct PointWithOffset {
    pub point: Point,
    pub x_offset: i32,
    pub y_offset: i32,
}

impl PointWithOffset {
    pub fn new(point: Point, x_offset: i32, y_offset: i32) -> PointWithOffset {
        PointWithOffset{
            point,
            x_offset,
            y_offset
        }
    }
}

impl Drawable for Ship {
    fn draw(&self, canvas: &mut WindowCanvas) -> () {
        debug!("Draw ship: {:#?}", self);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..self.points.len(){
            let next_index = (i+1) % self.points.len();
            canvas.draw_line(*self.points.get(i).unwrap().point, *self.points.get(next_index).unwrap().point).unwrap();
        }
    }
}

impl Ship {
    pub fn new() -> Ship {
        let xc = WIDTH/2;
        let yc = HEIGHT/2;

        let points = vec![
            PointWithOffset::new(Point::new(xc-DIMENSION ,yc+DIMENSION), -DIMENSION, DIMENSION),
            PointWithOffset::new(Point::new(xc+DIMENSION ,yc+DIMENSION), DIMENSION, DIMENSION),
            PointWithOffset::new(Point::new(xc ,yc-DIMENSION), 0, -DIMENSION)];
        Ship {
            position: PointExact{x: xc as f64 , y: yc as f64},
            angle: 0.0,
            points: points,
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
        self.angle -= 0.1;
        if self.angle < -PI {
            self.angle = PI;
        }
        self.compute_movements();
    }
    pub fn turn_left(&mut self) -> () {
        self.angle += 0.1;
        if self.angle > PI {
            self.angle = -PI;
        }
        self.compute_movements();
    }

    fn rotate_all(&mut self) -> () {
        for i in 0..self.points.len(){
            self.points[i].point = rotate(&self.points[i].point, &self.position, &self.angle);
        }
    }

    fn create_all(&mut self) -> () {
        for i in 0..self.points.len(){
            self.points[i].point = Point::new(self.position.x.round() as i32 + self.points[i].x_offset , self.position.y.round()  as i32 + self.points[i].y_offset);
        }
    }

    fn compute_movements(&mut self) -> () {
        self.create_all();
        self.rotate_all();
    }
}
