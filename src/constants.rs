use std::f32::consts::TAU;

use bevy::prelude::Color;

// Game constants
/// Show hitbox of objects
pub const GAME_SHOW_HIT_BOX: bool = false;
/// Name of the game
pub const GAME_NAME: &str = "⭐ Asteroids 🚀";
/// Name of the author
pub const GAME_AUTHOR: &str = "718";
/// email of the author
pub const GAME_AUTHOR_EMAIL: &str = "guilhem.radonde@gmail.com";
/// Name of testers
pub const GAME_TESTERS: [&str; 4] = ["Lilou", "V@V", "Sam", "Silyam"];
/// Frames rate in frames.s⁻¹
pub const GAME_FPS: u32 = 120;
/// Height of the window in px
pub const GAME_WINDOW_HEIGHT: f32 = 800.0;
/// Width of the window in px
pub const GAME_WINDOW_WIDTH: f32 = 800.0;
/// Max Y coordinates in px
pub const GAME_MAX_HEIGHT: f32 = GAME_WINDOW_HEIGHT / 2.0;
/// Max X coordinates in px
pub const GAME_MAX_WIDTH: f32 = GAME_WINDOW_WIDTH / 2.0;
/// Width of the lines of the meshes in px
pub const GAME_LINE_WIDTH: f32 = 1.0;
/// Y offset between each line of text diplayed
pub const GAME_TEXT_Y_OFFSET: f32 = 15.0;
/// Font to use for text
pub const GAME_FONT: &str = "fonts/DroidSans.ttf";

// Ship constants
pub const SHIP_SIZE_X: f32 = 7.0;
pub const SHIP_SIZE_Y: f32 = 7.0;
pub const SHIP_MASS: f32 = 30.0;
pub const SHIP_LIFE: f32 = 20.0;
pub const SHIP_INITIAL_DIRECTION: f32 = 0.0;
pub const SHIP_THRUST: f32 = 1.0;
pub const SHIP_TURN_THRUST: f32 = 50.0 * TAU;
pub const SHIP_MAX_SPEED: f32 = 500.0;
pub const SHIP_MAX_ANGLE_SPEED: f32 = 1.0 * TAU;
pub const SHIP_RELOAD_TIME: f32 = 1.;
pub const SHIP_DESCELERATION: f32 = 1.5;
pub const SHIP_ANGLE_DESCELERATION: f32 = 10.0;

// Bullet constants
pub const BULLET_SIZE: f32 = 0.5;
pub const BULLET_MASS: f32 = 1.0;
pub const BULLET_TIME_TO_LIVE: f32 = 2.0;
pub const BULLET_SPEED: f32 = 500.0;
pub const BULLET_DAMAGE: f32 = 100.0;

// Rock constants
pub const ROCK_RADIUS_INIT: f32 = 4.0 * ROCK_NB_EDGES as f32;
pub const ROCK_RADIUS_DECREMENT: f32 = 10.0;
pub const ROCK_RADIUS_DELTA: f32 = 10.0;
pub const ROCK_NB_EDGES: u32 = 10;
pub const ROCK_MIN_NB_EDGES: u32 = 8;
pub const ROCK_MASS: f32 = 500.0;
pub const ROCK_MASS_DECREMENT: f32 = 225.0;
pub const ROCK_LIFE: f32 = 100.0;
pub const ROCK_MAX_SPEED: f32 = 100.0;
pub const ROCK_MAX_ANGLE_SPEED: f32 = 1.0 * TAU;
pub const ROCK_COLLISION_DAMAGE: f32 = 20.0;

// Star constants
pub const STAR_NUMBER: usize = 100;
pub const STAR_RADIUS_MIN: f32 = 0.5;
pub const STAR_RADIUS_MAX: f32 = 1.5;

// Colors
pub const GREEN: Color = Color::rgb(0., 1., 0.);
pub const BLACK: Color = Color::rgb(0., 0., 0.);
