use crate::*;
use crate::{
    constants::*,
    inputs::{XDirection::*, YDirection::*},
    objects::{bullet::*, rock::*, ship::*, star::*, *},
    state::*,
};
use ggez::{
    graphics,
    graphics::{DrawParam, MeshBuilder, Text},
    timer, Context,
};
use log::{debug, info};

#[derive(Clone)]
pub enum NextState {
    Continue,
    Exit,
    Pause,
    GameOver,
}

#[derive(Clone)]
pub struct GameScreen {
    pub ship: Ship,
    pub rocks: Vec<Rock>,
    pub bullets: Vec<Bullet>,
    pub time_since_last_shoot: f32,
    pub stars: Vec<Star>,
    pub stage: u32,
    pub next_state: NextState,
}
impl GameScreen {
    pub fn new(ctx: &mut Context) -> GameScreen {
        let mut stars = vec![];
        for _ in 0..STAR_NUMBER {
            stars.push(Star::new(ctx))
        }
        // Load/create resources here: images, fonts, sounds, etc.
        GameScreen {
            ship: Ship::new(ctx),
            rocks: vec![Rock::new_init(ctx, ROCK_NB_EDGES, ROCK_RADIUS_INIT)],
            bullets: vec![],
            time_since_last_shoot: 0.0,
            stars: stars,
            stage: 1,
            next_state: NextState::Continue,
        }
    }

    fn draw_object(&self, ctx: &mut Context, obj: &impl Drawable) -> GameResult<()> {
        let obj_mesh = obj.get_mesh();
        let drawparams = graphics::DrawParam::new()
            .dest(*obj.get_position())
            .rotation(*obj.get_direction());
        graphics::draw(ctx, obj_mesh, drawparams)
    }

    fn draw_hitbox(&self, ctx: &mut Context, obj: &impl Collideable) -> GameResult<()> {
        if GAME_SHOW_HIT_BOX {
            let hitbox = MeshBuilder::default()
                .polygon(
                    graphics::DrawMode::stroke(GAME_LINE_WIDTH),
                    &[
                        Point::new(obj.get_hitbox().width / 2.0, obj.get_hitbox().height / 2.0),
                        Point::new(obj.get_hitbox().width / 2.0, -obj.get_hitbox().height / 2.0),
                        Point::new(
                            -obj.get_hitbox().width / 2.0,
                            -obj.get_hitbox().height / 2.0,
                        ),
                        Point::new(-obj.get_hitbox().width / 2.0, obj.get_hitbox().height / 2.0),
                    ],
                    GAME_HIT_BOX_COLOR,
                )?
                .to_owned()
                .build(ctx)?;
            let drawparams = DrawParam::new().dest(*obj.get_position());
            graphics::draw(ctx, &hitbox, drawparams)?;
        };
        Ok(())
    }

    fn draw_text(&self, ctx: &mut Context, txt: String, y_offset: f32) -> GameResult<()> {
        let display = Text::new(txt);
        graphics::draw(
            ctx,
            &display,
            (Point::new(-GAME_MAX_WIDTH, y_offset), GAME_TEXT_COLOR),
        )
    }

    fn draw_texts(&self, ctx: &mut Context) -> GameResult<()> {
        self.draw_text(
            ctx,
            format!("Current stage: {}", self.stage),
            -GAME_MAX_HEIGHT,
        )?;
        self.draw_text(
            ctx,
            format!("Life: {}/{}", self.ship.get_life(), SHIP_LIFE),
            -GAME_MAX_HEIGHT + GAME_TEXT_Y_OFFSET,
        )?;
        self.draw_text(
            ctx,
            format!("{} rocks to destroy", self.rocks.len()),
            -GAME_MAX_HEIGHT + 2.0 * GAME_TEXT_Y_OFFSET,
        )?;
        self.draw_text(
            ctx,
            format!("FPS: {:.0}", &timer::fps(ctx)),
            -GAME_MAX_HEIGHT + 3.0 * GAME_TEXT_Y_OFFSET,
        )?;
        Ok(())
    }

    pub fn shoot(&mut self, ctx: &mut Context) {
        debug!("Shoot the mofo !!!");
        let bullet = Bullet::new(ctx, *self.ship.get_position(), *self.ship.get_direction());
        self.bullets.push(bullet);
    }

    pub fn level_up(&mut self, ctx: &mut Context) {
        info!("Stage {} cleared", self.stage);
        self.stage += 1;
        for _ in 0..self.stage {
            self.rocks
                .push(Rock::new_init(ctx, ROCK_NB_EDGES, ROCK_RADIUS_INIT));
        }
        info!("New stage with {} rocks to shoot !", self.rocks.len())
    }

    pub fn game_over(&mut self, _ctx: &mut Context) {
        self.ship.explode();
        info!("Oups, you dead man !");
        self.next_state = NextState::GameOver;
    }
}

impl StateWithTransition for GameScreen {}

impl State for GameScreen {
    fn update(&mut self, ctx: &mut Context) {
        let time_elapsed = 1.0 / GAME_FPS as f32; //timer::delta(ctx).as_secs_f32();

        // Handle collisions with rocks
        for i in 0..self.rocks.len() {
            if self.ship.has_collided_with(&self.rocks[i]) {
                self.rocks[i].sub_life(ROCK_COLLISION_DAMAGE);
                self.ship.sub_life(ROCK_COLLISION_DAMAGE);
                let (ship_speed, rock_speed) = Ship::compute_speed_vectors_after_collision(
                    self.ship.get_speed(),
                    self.rocks[i].get_speed(),
                    self.ship.get_mass(),
                    self.rocks[i].get_mass(),
                    &self.ship.get_position().coords,
                    &self.rocks[i].get_position().coords,
                );
                self.ship.set_speed(ship_speed);
                self.rocks[i].set_speed(rock_speed);

                info!(
                    "Watchout, you collided with a rock. {} life remaining",
                    self.ship.get_life()
                );
            }

            for j in (i + 1)..self.rocks.len() {
                if i != j && self.rocks[i].has_collided_with(&self.rocks[j]) {
                    debug!("Colision between asteroid detected !!!");

                    let (rock1_speed, rock2_speed) = Rock::compute_speed_vectors_after_collision(
                        self.rocks[i].get_speed(),
                        self.rocks[j].get_speed(),
                        self.rocks[i].get_mass(),
                        self.rocks[j].get_mass(),
                        &self.rocks[i].get_position().coords,
                        &self.rocks[j].get_position().coords,
                    );
                    self.rocks[i].set_speed(rock1_speed);
                    self.rocks[j].set_speed(rock2_speed);
                }
            }
        }

        // Handle collisions with pewpew
        for i in 0..self.bullets.len() {
            for j in 0..self.rocks.len() {
                if self.bullets[i].has_collided_with(&self.rocks[j]) {
                    self.rocks[j].sub_life(BULLET_DAMAGE);
                    info!("Niiiice man ! You just hit a nasty rock dude !!!");
                    self.bullets[i].set_life(0.0);
                }
            }
        }

        // Handle shooting
        if self.ship.get_inputs().fire && self.time_since_last_shoot >= SHIP_RELOAD_TIME {
            self.shoot(ctx);
            self.time_since_last_shoot = 0.0;
        } else {
            self.time_since_last_shoot += time_elapsed;
        };

        // Handle ship
        match self.ship.get_inputs().yaxis {
            Forward => self.ship.accelerate(SHIP_THRUST, time_elapsed),
            Backward => self.ship.accelerate(-SHIP_THRUST, time_elapsed),
            _ => (),
        };
        match self.ship.get_inputs().xaxis {
            Right => self.ship.turn(SHIP_TURN_THRUST, time_elapsed),
            Left => self.ship.turn(-SHIP_TURN_THRUST, time_elapsed),
            _ => (),
        };
        self.ship.update_position(time_elapsed);
        self.ship.update_speeds(time_elapsed);

        // Handle bullets
        for bullet in &mut self.bullets {
            bullet.update_position(time_elapsed);
            bullet.sub_life(time_elapsed);
        }
        self.bullets
            .retain(|bullet| return bullet.get_life() > &0.0);

        // Handle rocks
        let mut new_litle_rocks: Vec<Rock> = vec![];
        for rock in &mut self.rocks {
            rock.update_position(time_elapsed);
            if rock.get_life() <= &0.0 && rock.get_nb_edges() > ROCK_MIN_NB_EDGES {
                new_litle_rocks.append(&mut rock.break_it(ctx));
            }
        }
        self.rocks.retain(|rock| return rock.get_life() > &0.0);
        self.rocks.append(&mut new_litle_rocks);

        // New stage ?
        if self.rocks.len() == 0 {
            self.level_up(ctx);
        }

        // Game over ?
        if self.ship.get_life() <= &0.0 {
            self.game_over(ctx)
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // Draw all the stars
        for star in &self.stars {
            self.draw_object(ctx, star)?;
        }

        // Draw all bullets
        for bullet in &self.bullets {
            self.draw_object(ctx, bullet)?;
            self.draw_hitbox(ctx, bullet)?;
        }

        // Draw all rocks
        for rock in &self.rocks {
            self.draw_object(ctx, rock)?;
            self.draw_hitbox(ctx, rock)?;
        }

        // Draw the ship
        self.draw_object(ctx, &self.ship)?;
        self.draw_hitbox(ctx, &self.ship)?;

        self.draw_texts(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Z => {
                self.ship.get_inputs().yaxis = Forward;
            }
            KeyCode::S => {
                self.ship.get_inputs().yaxis = Backward;
            }
            KeyCode::Q => {
                self.ship.get_inputs().xaxis = Left;
            }
            KeyCode::D => {
                self.ship.get_inputs().xaxis = Right;
            }
            KeyCode::Space => {
                self.ship.get_inputs().fire = true;
            }
            KeyCode::Return => {
                info!("Stop for a moment dude.");
                self.next_state = NextState::Pause;
            }
            KeyCode::Escape => {
                info!("Shit, the boss want to stop...");
                self.next_state = NextState::Exit;
            }
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Z | KeyCode::S => {
                self.ship.get_inputs().yaxis = YCenter;
            }
            KeyCode::Q | KeyCode::D => {
                self.ship.get_inputs().xaxis = XCenter;
            }
            KeyCode::Space => {
                self.ship.get_inputs().fire = false;
            }
            _ => (),
        }
    }
}

impl Transition for GameScreen {
    fn transition(&mut self, _ctx: &mut Context) -> Option<Box<dyn StateWithTransition>> {
        match self.next_state {
            NextState::Exit => Some(Box::new(StartScreen::new())),
            NextState::Pause => {
                self.next_state = NextState::Continue;
                Some(Box::new(StartScreen::new_with_game(self.clone())))
            }
            NextState::GameOver => Some(Box::new(GameOverScreen::new(self.stage))),
            _ => None,
        }
    }
}
