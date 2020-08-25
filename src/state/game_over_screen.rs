use ggez::{
    graphics,
    graphics::{Font, Scale, Text},
    input::keyboard::KeyCode,
    Context,
};

use crate::{constants::*, state::*, *};

pub struct GameOverScreen {
    pub next_state: bool,
    pub final_score: u32,
}

impl GameOverScreen {
    pub fn new(final_score: u32) -> GameOverScreen {
        GameOverScreen {
            next_state: false,
            final_score: final_score,
        }
    }
}

impl StateWithTransition for GameOverScreen {}

impl State for GameOverScreen {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut text = Text::new("Game Over.");
        text.set_font(Font::default(), Scale::uniform(80.0));
        let text_width = text.width(ctx) as f32;
        graphics::draw(
            ctx,
            &text,
            (Point::new(-text_width / 2.0, -80.0), GAME_TEXT_COLOR),
        )?;

        let mut text = Text::new(format!("You reached stage {}.", self.final_score));
        text.set_font(Font::default(), Scale::uniform(20.0));
        let text_width = text.width(ctx) as f32;
        graphics::draw(
            ctx,
            &text,
            (Point::new(-text_width / 2.0, 50.0), GAME_TEXT_COLOR),
        )?;

        let mut text = Text::new("You were a good person.");
        text.set_font(Font::default(), Scale::uniform(10.0));
        let text_width = text.width(ctx) as f32;
        graphics::draw(
            ctx,
            &text,
            (Point::new(-text_width / 2.0, 200.0), GAME_TEXT_COLOR),
        )?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        match keycode {
            KeyCode::Space | KeyCode::Return | KeyCode::Escape => {
                self.next_state = true;
            }
            _ => (),
        }
    }
}

impl Transition for GameOverScreen {
    fn transition(&mut self, _ctx: &mut Context) -> Option<Box<dyn StateWithTransition>> {
        match self.next_state {
            false => None,
            true => {
                info!("Let start from the beginning... again...");
                Some(Box::new(StartScreen::new()))
            }
        }
    }
}
