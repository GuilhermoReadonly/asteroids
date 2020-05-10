use ggez::{graphics, graphics::Text, input::keyboard::KeyCode};

use ggez::Context;

use crate::state::*;
use crate::*;

pub struct StartScreen {
    pub ready: bool,
    pub game: Option<GameScreen>,
}

impl StartScreen {
    pub fn new() -> StartScreen{
        StartScreen{ready: false, game: None}
    }

    pub fn new_with_game(game: GameScreen) -> StartScreen{
        StartScreen{ready: false, game: Some(game)}
    }
}

impl StateWithTransition for StartScreen {}

impl State for StartScreen {
    fn update(&mut self, _ctx: &mut Context) {}
    fn draw(&self, ctx: &mut Context) {
        let display = Text::new("Press Space to start.");
        graphics::draw(ctx, &display, (Point::new(0.0, 0.0), graphics::WHITE)).unwrap();
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space => {
                self.ready = true;
            }
            KeyCode::Escape => {
                event::quit(ctx);
            }
            _ => (),
        }
    }
}

impl Transition for StartScreen {
    fn transition(&mut self, ctx: &mut Context) -> Option<Box<dyn StateWithTransition>> {
        match (self.ready, self.game.clone()){
            (false, _) => None,
            (true, None) => Some(Box::new(GameScreen::new(ctx))),
            (true, Some(game)) => Some(Box::new(game)),
        }
    }
}
