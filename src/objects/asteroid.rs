use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::traits::Drawable;
use crate::traits::Moveable;
use crate::constants::*;
use crate::points::PointExact;
use crate::points::PointWithOffset;

#[derive(Debug, PartialEq)]
pub struct Asteroid {
    pub position: PointExact,
    pub angle: f64,
    pub points: Vec<PointWithOffset>,
}

impl Drawable for Asteroid {
    fn draw(&self, canvas: &mut WindowCanvas) -> () {
        debug!("Draw asteroid: {:#?}", self);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..self.points.len(){
            let next_index = (i+1) % self.points.len();
            canvas.draw_line(*self.points.get(i).unwrap().point, *self.points.get(next_index).unwrap().point).unwrap();
        }
    }
}

impl Moveable for Asteroid {
    fn set_position(&mut self, position: PointExact) -> (){
        self.position = position;
    }
    fn get_position(&self) -> &PointExact{
        &self.position
    }
    fn set_angle(&mut self, angle: f64) -> (){
        self.angle = angle;
    }
    fn get_angle(&self) -> &f64{
        &self.angle
    }
    fn set_points(&mut self, points: Vec<PointWithOffset>) -> (){
        self.points = points;
    }
    fn get_points(&self) -> &Vec<PointWithOffset>{
        &self.points
    }
    fn get_points_mut(&mut self) -> &mut Vec<PointWithOffset>{
        &mut self.points
    }
}

impl Asteroid {
    pub fn new() -> Asteroid {
        let xc = WIDTH/4;
        let yc = HEIGHT/4;

        let points = vec![
            PointWithOffset::new(Point::new(xc-DIMENSION ,yc+DIMENSION), -DIMENSION, DIMENSION),
            PointWithOffset::new(Point::new(xc+DIMENSION ,yc+DIMENSION), DIMENSION, DIMENSION),
            PointWithOffset::new(Point::new(xc+DIMENSION/2 ,yc), DIMENSION/2, 0),
            PointWithOffset::new(Point::new(xc ,yc-DIMENSION), 0, -DIMENSION),
            PointWithOffset::new(Point::new(xc-DIMENSION/2 ,yc), -DIMENSION/2, 0)];
        Asteroid {
            position: PointExact{x: xc as f64 , y: yc as f64},
            angle: 0.0,
            points: points,
        }
    }

}
