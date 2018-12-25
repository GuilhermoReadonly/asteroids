use sdl2::render::WindowCanvas;

use crate::objects::SpaceObject;
use crate::traits::Drawable;

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
    pub fn new() -> Universe {
        let asteroids = vec![SpaceObject::new_asteroid(8)];

        Universe{
            player: SpaceObject::new_ship(),
            asteroids: asteroids,
        }
    }
}
