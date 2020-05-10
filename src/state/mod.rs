mod start_screen;
pub use start_screen::StartScreen;

mod game_screen;
pub use game_screen::GameScreen;

use ggez::{
    event::{KeyCode, KeyMods},
    Context,
};

pub trait StateWithTransition: State + Transition {}

pub trait State {
    fn update(&mut self, ctx: &mut Context);
    fn draw(&self, ctx: &mut Context);
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
    }
    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymod: KeyMods) {}
}

pub trait Transition {
    fn transition(&mut self, ctx: &mut Context) -> Option<Box<dyn StateWithTransition>>;
}
