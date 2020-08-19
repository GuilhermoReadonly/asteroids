pub mod constants;
pub mod inputs;
pub mod objects;
pub mod state;

use crate::{constants::*, state::*};
use ggez::{
    event, graphics,
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};

use log::info;

pub type SpeedVector = Vector2<f32>;
pub type SpeedScalar = f32;
pub type SpeedAngle = f32;
pub type PositionVector = Vector2<f32>;
pub type Point = Point2<f32>;
pub type Direction = f32;
pub type DirectionVector = Vector2<f32>;
pub type Mass = f32;
pub type Force = f32;
pub type Life = f32;

pub struct MainState {
    state: Box<dyn StateWithTransition>,
}

impl MainState {
    pub fn new() -> MainState {
        info!("Let the party rocks !");
        MainState {
            state: Box::new(StartScreen::new()),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, GAME_FPS) {
            self.state.update(ctx);
        }
        if let Some(new_state) = self.state.transition(ctx) {
            self.state = new_state
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.state.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    // Handle key events.  These just map keyboard events
    // and alter our input state appropriately.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        keymod: event::KeyMods,
        repeat: bool,
    ) {
        self.state.key_down_event(ctx, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, keymod: event::KeyMods) {
        self.state.key_up_event(ctx, keycode, keymod);
    }
}
