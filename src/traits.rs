use sdl2::render::WindowCanvas;
use sdl2::rect::Point;

use crate::points::PointExact;
use crate::points::PointWithOffset;
use crate::math::rotate;
use crate::math::translate;
use crate::constants::*;

pub trait Drawable {
    fn draw(&self, canvas: & mut WindowCanvas) -> ();
}

pub trait Moveable {

    fn set_position(&mut self, position: PointExact) -> ();
    fn get_position(&self) -> &PointExact;

    fn set_angle(&mut self, angle: f64) -> ();
    fn get_angle(&self) -> &f64;

    fn set_speed(&mut self, angle: f64) -> ();
    fn get_speed(&self) -> &f64;

    fn set_points(&mut self, points: Vec<PointWithOffset>) -> ();
    fn get_points(&self) -> &Vec<PointWithOffset>;
    fn get_points_mut(&mut self) -> &mut Vec<PointWithOffset>;

    fn do_nothing(&mut self) -> () {
        self.set_position(translate(&self.get_position(), self.get_speed(), &self.get_angle()));
        self.compute_movements();
    }

    fn move_up(&mut self) -> () {
        self.set_speed(self.get_speed() + &SPEED_STEP);
        self.set_position(translate(self.get_position(), self.get_speed(), self.get_angle()));
        self.compute_movements();
    }
    fn move_down(&mut self) -> () {
        self.set_speed(self.get_speed() - &SPEED_STEP);
        self.set_position(translate(&self.get_position(), self.get_speed(), &self.get_angle()));
        self.compute_movements();
    }
    fn turn_right(&mut self) -> () {
        self.set_angle((self.get_angle() - STEP_ROTATE) % PI_2);
        self.compute_movements();
    }
    fn turn_left(&mut self) -> () {
        self.set_angle((self.get_angle() + STEP_ROTATE) % PI_2);
        self.compute_movements();
    }

    fn rotate_all(&mut self) -> () {
        for i in 0..self.get_points().len(){
            self.get_points_mut()[i].point = rotate(&self.get_points()[i].point, &self.get_position(), &self.get_angle());
        }
    }

    fn create_all(&mut self) -> () {
        for i in 0..self.get_points().len(){
            self.get_points_mut()[i].point = Point::new(self.get_position().x.round() as i32 + self.get_points()[i].x_offset , self.get_position().y.round()  as i32 + self.get_points()[i].y_offset);
        }
    }

    fn compute_movements(&mut self) -> () {
        self.create_all();
        self.rotate_all();
    }
}
