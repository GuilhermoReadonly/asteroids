use crate::{
    constants::*,
    inputs::{InputState, XDirection::*, YDirection::*},
    objects::{bullet::Bullet, rock::Rock, ship::Ship, Object, Point},
};
use ggez::{
    event,
    event::{EventHandler, KeyCode, KeyMods},
    graphics,
    graphics::Color,
    timer, Context, GameResult,
};
use log::info;

pub struct AsteroidWorld {
    pub ship: Ship,
    pub rocks: Vec<Rock>,
    pub bullets: Vec<Ship>,
    pub input: InputState,
    pub time_since_last_shoot: f32,
}

impl AsteroidWorld {
    pub fn new(ctx: &mut Context) -> AsteroidWorld {
        // Load/create resources here: images, fonts, sounds, etc.
        AsteroidWorld {
            ship: Ship::new_ship(ctx),
            rocks: vec![Rock::new_rock(ctx)],
            bullets: vec![],
            input: InputState::default(),
            time_since_last_shoot: 0.0,
        }
    }

    fn draw_object(&self, ctx: &mut Context, obj: &Object) -> GameResult<()> {
        let obj_mesh = &obj.mesh;

        let drawparams = graphics::DrawParam::new()
            .dest(world_to_screen_coords(&obj.position))
            .rotation(obj.direction);
        graphics::draw(ctx, obj_mesh, drawparams)
    }

    pub fn shoot(&mut self, ctx: &mut Context) {
        info!("Shoot the mofo !!!");
        let bullet = Bullet::new_bullet(ctx, self.ship.position, self.ship.direction);
        self.bullets.push(bullet);
    }
}

impl EventHandler for AsteroidWorld {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, GAME_FPS) {
            let time_elapsed = 1.0 / (GAME_FPS as f32);

            // Handle shooting
            if self.input.fire && self.time_since_last_shoot >= SHIP_RELOAD_TIME {
                self.shoot(ctx);
                self.time_since_last_shoot = 0.0;
            } else {
                self.time_since_last_shoot += time_elapsed;
            };

            // Handle ship
            match self.input.yaxis {
                Forward => self.ship.accelerate(SHIP_THRUST, time_elapsed),
                Backward => self.ship.accelerate(-SHIP_THRUST, time_elapsed),
                _ => (),
            };
            match self.input.xaxis {
                Right => self.ship.turn(SHIP_TURN_INCREMENT, time_elapsed),
                Left => self.ship.turn(-SHIP_TURN_INCREMENT, time_elapsed),
                _ => (),
            };
            if self.ship.life <= 0.0 {
                self.ship.explode();
                let _ = event::quit(ctx);
            }
            self.ship.update_position(time_elapsed);

            // Handle bullets
            for bullet in &mut self.bullets {
                bullet.update_position(time_elapsed);
                bullet.update_life(time_elapsed);
            }
            self.bullets.retain(|bullet| return bullet.life > 0.0);

            // Handle rocks
            for rock in &mut self.rocks {
                rock.update_position(time_elapsed);
            }
            self.rocks.retain(|rock| return rock.life > 0.0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 0.0));

        // Draw the ship
        self.draw_object(ctx, &self.ship)?;

        // Draw all bullets
        for bullet in &self.bullets {
            self.draw_object(ctx, bullet)?;
        }

        // Draw all rocks
        for rock in &self.rocks {
            self.draw_object(ctx, rock)?;
        }

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

/// Translates the world coordinate system, which
/// has Y pointing up and the origin at the center,
/// to the screen coordinate system, which has Y
/// pointing downward and the origin at the top-left,
fn world_to_screen_coords(point: &Point) -> Point {
    let x = point.x + GAME_WINDOW_WIDTH / 2.0;
    let y = GAME_WINDOW_HEIGHT - (point.y + GAME_WINDOW_HEIGHT / 2.0);
    Point::new(x, y)
}
