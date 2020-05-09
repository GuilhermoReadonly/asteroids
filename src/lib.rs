pub mod constants;
pub mod inputs;
pub mod objects;

use crate::{
    constants::*,
    inputs::{XDirection::*, YDirection::*},
    objects::{bullet::*, rock::*, ship::*, star::*, *},
};
use ggez::{
    event,
    event::{EventHandler, KeyCode, KeyMods},
    graphics,
    graphics::{Color, DrawParam, MeshBuilder, Text},
    timer, Context, GameResult,
};
use log::{debug, info};

pub struct AsteroidWorld {
    pub ship: Ship,
    pub rocks: Vec<Rock>,
    pub bullets: Vec<Bullet>,
    pub time_since_last_shoot: f32,
    pub stars: Vec<Star>,
    pub stage: u32,
}

impl AsteroidWorld {
    pub fn new(ctx: &mut Context) -> AsteroidWorld {
        let mut stars = vec![];
        for _ in 0..STAR_NUMBER {
            stars.push(Star::new(ctx))
        }
        // Load/create resources here: images, fonts, sounds, etc.
        AsteroidWorld {
            ship: Ship::new(ctx),
            rocks: vec![Rock::new_init(ctx, ROCK_NB_EDGES, ROCK_RADIUS_INIT)],
            bullets: vec![],
            time_since_last_shoot: 0.0,
            stars: stars,
            stage: 1,
        }
    }

    fn draw_object(&self, ctx: &mut Context, obj: &impl Drawable) -> GameResult<()> {
        let obj_mesh = obj.get_mesh();
        let drawparams = graphics::DrawParam::new()
            .dest(world_to_screen_coords(&obj.get_position()))
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
                )
                .unwrap()
                .to_owned()
                .build(ctx)
                .unwrap();
            let drawparams = DrawParam::new().dest(world_to_screen_coords(&obj.get_position()));
            graphics::draw(ctx, &hitbox, drawparams)?;
        };
        Ok(())
    }

    fn draw_text(&self, ctx: &mut Context, txt: String, y_offset: f32) -> GameResult<()> {
        let display = Text::new(txt);
        graphics::draw(ctx, &display, (Point::new(0.0, y_offset), graphics::WHITE))
    }

    fn draw_texts(&self, ctx: &mut Context) -> GameResult<()> {
        self.draw_text(ctx, format!("Current stage: {}", self.stage), 0.0)?;
        self.draw_text(
            ctx,
            format!("Life: {}/{}", self.ship.get_life(), SHIP_LIFE),
            GAME_TEXT_Y_OFFSET,
        )?;
        self.draw_text(
            ctx,
            format!("{} rocks to destroy", self.rocks.len()),
            2.0 * GAME_TEXT_Y_OFFSET,
        )?;
        self.draw_text(
            ctx,
            format!("FPS: {:.0}", &timer::fps(ctx)),
            3.0 * GAME_TEXT_Y_OFFSET,
        )
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

    pub fn game_over(&mut self, ctx: &mut Context) {
        self.ship.explode();
        info!("Oups, you dead man !");
        event::quit(ctx);
    }
}

impl EventHandler for AsteroidWorld {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let time_elapsed = timer::delta(ctx).as_secs_f32();

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
        let mut new_litle_rocks: Vec<Box<Rock>> = vec![];
        for rock in &mut self.rocks {
            rock.update_position(time_elapsed);
            if rock.get_life() <= &0.0 && rock.get_nb_edges() > ROCK_MIN_NB_EDGES {
                new_litle_rocks = rock.break_it(ctx);
            }
        }
        self.rocks.retain(|rock| return rock.get_life() > &0.0);
        let mut new_litle_rocks = new_litle_rocks.iter().map(|r| *r.clone()).collect();
        self.rocks.append(&mut new_litle_rocks);

        // New stage ?
        if self.rocks.len() == 0 {
            self.level_up(ctx);
        }

        // Game over ?
        if self.ship.get_life() <= &0.0 {
            self.game_over(ctx)
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 0.0));

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

        self.draw_texts(ctx)?;

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

/// Translates the world coordinate system, which
/// has Y pointing up and the origin at the center,
/// to the screen coordinate system, which has Y
/// pointing downward and the origin at the top-left,
fn world_to_screen_coords(point: &Point) -> Point {
    let x = point.x + GAME_WINDOW_WIDTH / 2.0;
    let y = GAME_WINDOW_HEIGHT - (point.y + GAME_WINDOW_HEIGHT / 2.0);
    Point::new(x, y)
}
