use ggez::{
    graphics,
    graphics::{Font, Scale, Text},
    input::keyboard::KeyCode,
    Context,
};

use crate::{constants::*, state::*, *};

#[derive(Clone)]
pub enum NextState {
    Continue,
    Play,
    Credits,
}

pub struct StartScreen {
    pub next_state: NextState,
    pub game: Option<GameScreen>,
}

impl StartScreen {
    pub fn new() -> StartScreen {
        StartScreen {
            next_state: NextState::Continue,
            game: None,
        }
    }

    pub fn new_with_game(game: GameScreen) -> StartScreen {
        StartScreen {
            next_state: NextState::Continue,
            game: Some(game),
        }
    }
}

impl StateWithTransition for StartScreen {}

impl State for StartScreen {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut start_text = Text::new(format!(
            "Press Enter to {}, Esc to quit.",
            if self.game.is_none() {
                "start"
            } else {
                "continue"
            }
        ));
        start_text.set_font(Font::default(), Scale::uniform(40.0));
        let text_width = start_text.width(ctx) as f32;
        graphics::draw(
            ctx,
            &start_text,
            (Point::new(-text_width / 2.0, -100.0), GAME_TEXT_COLOR),
        )?;

        let mut ctrl_text = Text::new("Controls : Z, S, Q, D, Space.");
        ctrl_text.set_font(Font::default(), Scale::uniform(20.0));
        let text_width = ctrl_text.width(ctx) as f32;
        graphics::draw(
            ctx,
            &ctrl_text,
            (Point::new(-text_width / 2.0, 50.0), GAME_TEXT_COLOR),
        )?;
        let mut ctrl_text = Text::new("Pause in game : Enter");
        ctrl_text.set_font(Font::default(), Scale::uniform(20.0));
        let text_width = ctrl_text.width(ctx) as f32;
        graphics::draw(
            ctx,
            &ctrl_text,
            (Point::new(-text_width / 2.0, 100.0), GAME_TEXT_COLOR),
        )?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        match keycode {
            KeyCode::Return => {
                self.next_state = NextState::Play;
            }
            KeyCode::F7 => {
                self.next_state = NextState::Credits;
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
        match (self.next_state.clone(), self.game.clone()) {
            (NextState::Credits, _) => {
                info!("Hooo, you want to know me better ?");
                Some(Box::new(CreditsScreen::new()))
            }
            (NextState::Play, None) => {
                info!("Let's break some rocks !!!");
                Some(Box::new(GameScreen::new(ctx)))
            }
            (NextState::Play, Some(game)) => {
                info!("Where were we at already ?");
                Some(Box::new(game))
            }
            (_, _) => None,
        }
    }
}
