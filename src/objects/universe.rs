use sdl2::render::WindowCanvas;

use crate::objects::ship::Ship;
use crate::objects::asteroid::Asteroid;
use crate::drawable::Drawable;

#[derive(Debug, PartialEq)]
pub struct Universe {
    pub player: Ship,
    pub asteroids: Vec<Asteroid>,
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
        let asteroids = vec![Asteroid::new()];

        Universe{
            player: Ship::new(),
            asteroids: asteroids,
        }
    }
}
