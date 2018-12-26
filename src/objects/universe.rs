use sdl2::render::WindowCanvas;
use rand::Rng;

use crate::objects::SpaceObject;
use crate::traits::Moveable;
use crate::traits::Drawable;
use crate::constants::*;

#[derive(Debug, PartialEq)]
pub struct Universe {
    pub player: SpaceObject,
    pub asteroids: Vec<SpaceObject>,
}

impl Drawable for Universe {
    fn draw(&self, canvas: &mut WindowCanvas) -> () {
        self.player.draw(canvas);

        for i in 0..self.asteroids.len(){
            self.asteroids[i].draw(canvas);
        }
    }
}

impl Universe {
    pub fn new(asteroid_number: u32) -> Universe {
        let mut rng = rand::thread_rng();

        let mut asteroids = vec![];

        for _i in 0..asteroid_number{
            let x: f64 = rng.gen::<f64>() * WIDTH as f64;
            let y: f64 = rng.gen::<f64>() * HEIGHT as f64;
            asteroids.push(SpaceObject::new_asteroid(x, y, ASTEROID_INIT_SIZE));
        }

        let x: f64 = rng.gen::<f64>() * WIDTH as f64;
        let y: f64 = rng.gen::<f64>() * HEIGHT as f64;
        let angle: f64 = rng.gen::<f64>() * PI_2;
        Universe{
            player: SpaceObject::new_ship(x, y, angle),
            asteroids: asteroids,
        }
    }

    pub fn compute_positions(&mut self) -> () {
        self.player.compute_position();
        for i in 0..self.asteroids.len(){
            self.asteroids[i].compute_position();
        }
    }

}
