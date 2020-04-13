use log::trace;

use crate::{
    constants::{ARENA_HEIGHT, ARENA_WIDTH, SHIP_COLOR},
    objects::ship::Ship,
};

use ggez::{event::EventHandler, graphics, graphics::Color, nalgebra::Point2, Context, GameResult};

pub struct AsteroidGame {
    ship: Ship,
}

impl AsteroidGame {
    pub fn new(_ctx: &mut Context) -> AsteroidGame {
        // Load/create resources here: images, fonts, sounds, etc.
        AsteroidGame { ship: Ship::new() }
    }

    pub fn draw_ship(&mut self, ctx: &mut Context) -> GameResult<()> {
        let ship = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::stroke(0.5),
            &self.ship.perimeter,
            SHIP_COLOR,
        )?;

        let pos: Point2<f32> = self.ship.position.into();
        // let pos: Point2<f32> = Point2::new(400.0,400.0);
        let drawparams = graphics::DrawParam::new().dest(pos).rotation(0.0 as f32);
        //.offset(Point2::new(ARENA_WIDTH, ARENA_HEIGHT));

        graphics::draw(ctx, &ship, drawparams)
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
        self.draw_ship(ctx)?;

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
