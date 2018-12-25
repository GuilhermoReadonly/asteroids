use sdl2::render::WindowCanvas;

use crate::objects::SpaceObject;
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
        let mut asteroids = vec![];

        for _i in 0..asteroid_number{
            asteroids.push(SpaceObject::new_asteroid(ASTEROID_INIT_SIZE));
        }

        Universe{
            player: SpaceObject::new_ship(),
            asteroids: asteroids,
        }
    }
}
