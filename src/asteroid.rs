use crate::objects::ship::Ship;
use ggez::{event::EventHandler, graphics, graphics::Color, Context, GameResult};
use log::trace;

pub struct AsteroidGame {
    ship: Ship,
}

impl AsteroidGame {
    pub fn new(_ctx: &mut Context) -> AsteroidGame {
        // Load/create resources here: images, fonts, sounds, etc.
        AsteroidGame {
            ship: Ship::new_ship(),
        }
    }
}

impl EventHandler for AsteroidGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        trace!("update event {:?}", ctx);
        self.ship.move_forward(0.1);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 0.0));

        trace!("draw my ship {:?}", ctx);
        self.ship.draw(ctx)?;

        graphics::present(ctx)
    }
}

// /// Translates the world coordinate system, which
// /// has Y pointing up and the origin at the center,
// /// to the screen coordinate system, which has Y
// /// pointing downward and the origin at the top-left,
// fn world_to_screen_coords(point: Point2<f32>) -> Point2<f32> {
//     let x = point.x + ARENA_WIDTH / 2.0;
//     let y = ARENA_HEIGHT - (point.y + ARENA_HEIGHT / 2.0);
//     Point2::new(x, y)
// }
