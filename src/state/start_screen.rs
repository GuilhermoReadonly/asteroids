use ggez::{
    graphics,
    graphics::{Font, Scale, Text},
    input::keyboard::KeyCode,
    Context,
};

use crate::{constants::*, state::*, *};

pub struct StartScreen {
    pub ready: bool,
    pub game: Option<GameScreen>,
}

impl StartScreen {
    pub fn new() -> StartScreen {
        StartScreen {
            ready: false,
            game: None,
        }
    }

    pub fn new_with_game(game: GameScreen) -> StartScreen {
        StartScreen {
            ready: false,
            game: Some(game),
        }
    }
}

impl StateWithTransition for StartScreen {}

impl State for StartScreen {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&self, ctx: &mut Context) {
        let mut start_text = Text::new(format!(
            "Press Space or Enter to {}.",
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
            (Point::new(-text_width / 2.0, -100.0), GAME_COLOR_WHITE),
        )
        .unwrap();

        let mut ctrl_text = Text::new("Controls : Z, S, Q, D, Space.");
        ctrl_text.set_font(Font::default(), Scale::uniform(20.0));
        let text_width = ctrl_text.width(ctx) as f32;
        graphics::draw(
            ctx,
            &ctrl_text,
            (Point::new(-text_width / 2.0, 50.0), GAME_COLOR_WHITE),
        )
        .unwrap();
        let mut ctrl_text = Text::new("Pause in game : Enter");
        ctrl_text.set_font(Font::default(), Scale::uniform(20.0));
        let text_width = ctrl_text.width(ctx) as f32;
        graphics::draw(
            ctx,
            &ctrl_text,
            (Point::new(-text_width / 2.0, 100.0), GAME_COLOR_WHITE),
        )
        .unwrap();
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        match keycode {
            KeyCode::Space | KeyCode::Return => {
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
        match (self.ready, self.game.clone()) {
            (false, _) => None,
            (true, None) => {
                info!("Let's break some rocks !!!");
                Some(Box::new(GameScreen::new(ctx)))
            }
            (true, Some(game)) => {
                info!("Where were we at already ?");
                Some(Box::new(game))
            }
        }
    }
}
