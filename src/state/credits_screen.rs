use ggez::{
    graphics,
    graphics::{Font, Scale, Text},
    input::keyboard::KeyCode,
    Context,
};

use crate::{constants::*, state::*, *};

pub struct CreditsScreen {
    pub next_state: bool,
}

impl CreditsScreen {
    pub fn new() -> CreditsScreen {
        CreditsScreen { next_state: false }
    }

    fn draw_text(ctx: &mut Context, text: String, font_scale: f32, y: f32) -> GameResult<()> {
        let mut text = Text::new(text);

        text.set_font(Font::default(), Scale::uniform(font_scale));
        let text_width = text.width(ctx) as f32;

        graphics::draw(
            ctx,
            &text,
            (Point::new(-text_width / 2.0, y), GAME_TEXT_COLOR),
        )
    }
}

impl StateWithTransition for CreditsScreen {}

impl State for CreditsScreen {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        Self::draw_text(
            ctx,
            format!("This stupid game was lazily created by {}.", GAME_AUTHOR),
            20.0,
            -100.0,
        )?;

        Self::draw_text(
            ctx,
            "It was inconsistently tested with very low efforts by these people :".to_string(),
            20.0,
            0.0,
        )?;

        Self::draw_text(ctx, format!("{}.", GAME_TESTERS.join(" and ")), 20.0, 30.0)?;

        Self::draw_text(
            ctx,
            "If you want to insult me about this crap you can do so at :".to_string(),
            10.0,
            100.0,
        )?;

        Self::draw_text(ctx, format!("{}.", GAME_AUTHOR_EMAIL), 10.0, 110.0)
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

impl Transition for CreditsScreen {
    fn transition(&mut self, _ctx: &mut Context) -> Option<Box<dyn StateWithTransition>> {
        match self.next_state {
            false => None,
            true => {
                info!("Okey, we had fun, go back to the serious stuffs now !");
                Some(Box::new(StartScreen::new()))
            }
        }
    }
}
