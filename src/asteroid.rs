use log::{trace};

use crate::{
    constants::{SHIP_COLOR},
    objects::ship::Ship,
};

use ggez::{
    event::EventHandler,
    graphics,
    graphics::Color,
    nalgebra::Point2,
    Context, GameResult,
};

pub struct AsteroidGame {
    ship: Ship,
}

impl AsteroidGame {
    pub fn new(_ctx: &mut Context) -> AsteroidGame {
        // Load/create resources here: images, fonts, sounds, etc.
        AsteroidGame { ship: Ship::new() }
    }

    pub fn draw_ship(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Load/create resources here: images, fonts, sounds, etc.
        let ship = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &self.ship.perimeter,
            SHIP_COLOR,
        )?;

        graphics::draw(ctx, &ship, (Point2::new(0.0, 0.0),))
    }
}

impl EventHandler for AsteroidGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        trace!("update event {:?}", ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 0.0));
        trace!("draw my ship {:?}", ctx);
        self.draw_ship(ctx)?;

        graphics::present(ctx)
    }
}
