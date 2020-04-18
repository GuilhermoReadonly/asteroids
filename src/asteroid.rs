use crate::{
    constants::*,
    inputs::{InputState, XDirection::*, YDirection::*},
    objects::ship::Ship,
};
use ggez::{
    event,
    event::{EventHandler, KeyCode, KeyMods},
    graphics,
    graphics::Color,
    timer, Context, GameResult,
};
use log::{info, trace};

pub struct AsteroidGame {
    ship: Ship,
    input: InputState,
}

impl AsteroidGame {
    pub fn new(_ctx: &mut Context) -> AsteroidGame {
        // Load/create resources here: images, fonts, sounds, etc.
        AsteroidGame {
            ship: Ship::new_ship(),
            input: InputState::default(),
        }
    }
}

impl EventHandler for AsteroidGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, GAME_FPS) {
            let _seconds = 1.0 / (GAME_FPS as f32);

            // Update the player state based on the user input.
            match self.input.yaxis {
                Forward => self.ship.accelerate(SHIP_SPEED_INCREMENT),
                Backward => self.ship.accelerate(-SHIP_SPEED_INCREMENT),
                _ => (),
            };
            match self.input.xaxis {
                Left => self.ship.turn(SHIP_TURN_INCREMENT),
                Right => self.ship.turn(-SHIP_TURN_INCREMENT),
                _ => (),
            };

            if self.input.fire {
                self.ship.shoot();
            };

            // Finally we check for our end state.
            // I want to have a nice death screen eventually,
            // but for now we just quit.
            if self.ship.life <= 0.0 {
                self.ship.explode();
                let _ = event::quit(ctx);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 0.0));

        trace!("draw my ship {:?}", ctx);
        self.ship.draw(ctx)?;

        graphics::present(ctx)
    }

    // Handle key events.  These just map keyboard events
    // and alter our input state appropriately.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Z => {
                self.input.yaxis = Forward;
            }
            KeyCode::S => {
                self.input.yaxis = Backward;
            }
            KeyCode::Q => {
                self.input.xaxis = Left;
            }
            KeyCode::D => {
                self.input.xaxis = Right;
            }
            KeyCode::Space => {
                self.input.fire = true;
            }
            KeyCode::P => {
                let img = graphics::screenshot(ctx).expect("Could not take screenshot");
                img.encode(ctx, graphics::ImageFormat::Png, "./screenshot.png")
                    .expect("Could not save screenshot");
            }
            KeyCode::Escape => {
                info!("Shit, the boss want to stop...");
                event::quit(ctx)
            }
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Z | KeyCode::S => {
                self.input.yaxis = YCenter;
            }
            KeyCode::Q | KeyCode::D => {
                self.input.xaxis = XCenter;
            }
            KeyCode::Space => {
                self.input.fire = false;
            }
            _ => (),
        }
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
