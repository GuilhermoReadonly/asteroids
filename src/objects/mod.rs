pub mod universe;

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use rand::Rng;

use crate::traits::Drawable;
use crate::traits::Moveable;
use crate::constants::*;
use crate::points::PointExact;
use crate::points::PointWithOffset;
use crate::math::rotate_point_with_offset;

#[derive(Debug, PartialEq)]
pub struct SpaceObject {
    pub position: PointExact,
    pub angle: f64,
    pub speed: f64,
    pub radius: f64,
    pub points: Vec<PointWithOffset>,
}

impl Drawable for SpaceObject {
    fn draw(&self, canvas: &mut WindowCanvas) -> () {
        debug!("Draw SpaceObject: {:#?}", self);
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        for i in 0..self.points.len(){
            let next_index = (i+1) % self.points.len();
            canvas.draw_line(*self.points.get(i).unwrap().point, *self.points.get(next_index).unwrap().point).unwrap();
        }
    }
}

impl Moveable for SpaceObject {
    fn set_position(&mut self, position: PointExact) -> (){
        let x = if position.x > WIDTH as f64{0.0}
        else if position.x < 0.0 {WIDTH as f64}
        else {position.x};

        let y = if position.y > HEIGHT as f64 {0.0}
        else if position.y < 0.0 {HEIGHT as f64}
        else {position.y};

        let new_position = PointExact::new(x, y);
        self.position = new_position;
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
    fn set_speed(&mut self, speed: f64) -> (){
        self.speed = speed.min(SPEED_MAX).max(SPEED_MIN);
    }
    fn get_speed(&self) -> &f64{
        &self.speed
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

impl SpaceObject {
    pub fn new_ship(x: f64, y: f64) -> SpaceObject {
        let xc = x as i32;
        let yc =y as i32;

        let points = vec![
            PointWithOffset::new(xc ,yc , -DIMENSION, DIMENSION),
            PointWithOffset::new(xc ,yc, 0, DIMENSION/2),
            PointWithOffset::new(xc ,yc, DIMENSION, DIMENSION),
            PointWithOffset::new(xc ,yc, 0, -DIMENSION)];

        SpaceObject {
            position: PointExact{x: xc as f64 , y: yc as f64},
            angle: 0.0,
            speed: 0.0,
            radius: DIMENSION as f64,
            points: points,
        }
    }

    pub fn new_asteroid(x: f64, y: f64, size: u32) -> SpaceObject {
        let mut rng = rand::thread_rng();
        let xc: f64 = x;
        let yc: f64 = y;

        let center_of_asteroid = PointExact::new(xc, yc);
        let mut points = vec![];
        let rotation_step = PI_2/size as f64;

        for i in 0..size{
            let initial_point_to_rotate = PointWithOffset::new(xc as i32 ,yc as i32 , DIMENSION + (rng.gen::<f64>() * ASTEROID_RANDOMNESS_FACTOR) as i32, DIMENSION + (rng.gen::<f64>() * ASTEROID_RANDOMNESS_FACTOR) as i32);
            let point = rotate_point_with_offset(&initial_point_to_rotate, &center_of_asteroid, &(i as f64 * rotation_step));
            points.push(point);
        }

        SpaceObject {
            position: PointExact{x: xc as f64 , y: yc as f64},
            angle: rng.gen::<f64>() * PI_2,
            speed: ASTEROID_INIT_SPEED,
            radius: DIMENSION as f64,
            points: points,
        }
    }

    pub fn distance_to(&self, other_object: &SpaceObject) -> f64{
        ((self.position.x - other_object.position.x).powi(2) + (self.position.y - other_object.position.y).powi(2)).sqrt()
    }

    pub fn has_collided_with(&self, other_object: &SpaceObject) -> bool{
        let mut result = false;
        if self.distance_to(other_object) < self.radius + other_object.radius{
            result = true;
        }
        result
    }
}



#[test]
fn is_colision_happened_test(){
    //almost don't collide
    let ship = SpaceObject::new_ship(0.0, 0.0);
    let asteroid = SpaceObject::new_asteroid(ship.radius * 2.0 - 1.0, 0.0, 12);
    let result = ship.has_collided_with(&asteroid);
    assert_eq!(result, true);

    //almost collide
    let ship = SpaceObject::new_ship(0.0, 0.0);
    let asteroid = SpaceObject::new_asteroid(ship.radius * 2.0, 0.0, 12);
    let result = ship.has_collided_with(&asteroid);
    assert_eq!(result, false);
}

#[test]
fn turn_right_test(){
    let mut object = SpaceObject::new_ship(0.0, 0.0);

    object.turn_right();
    assert_eq!(object.angle, -STEP_ROTATE);

    for _i in 0..64{
        object.turn_right();
    }
    assert!(object.angle - STEP_ROTATE < std::f64::EPSILON);
}

#[test]
fn turn_left_test(){
    let mut object = SpaceObject::new_ship(0.0, 0.0);

    object.turn_left();
    assert_eq!(object.angle, STEP_ROTATE);

    for _i in 0..64{
        object.turn_left();
    }
    assert!(object.angle - STEP_ROTATE < std::f64::EPSILON);
}
