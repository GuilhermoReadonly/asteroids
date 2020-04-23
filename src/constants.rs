use ggez::graphics::Color;
use std::f32::consts::PI;

// Game constants

/// The full circle constant (τ)
pub const TAU: f32 = 2.0 * PI;

// Game constants
/// Show hitbox radius of objects
pub const GAME_SHOW_HIT_BOX: bool = false;
// The color of the hit box if activated
pub const GAME_HIT_BOX_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);
/// Name of the game
pub const GAME_NAME: &str = "⭐ Asteroids 🚀";
/// Name of the author
pub const GAME_AUTHOR: &str = "718";
/// Frames rate in frames.s⁻¹
pub const _GAME_FPS: u32 = 120;
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

// Ship constants
pub const SHIP_SIZE_X: f32 = 7.0;
pub const SHIP_SIZE_Y: f32 = 7.0;
pub const SHIP_MASS: f32 = 30.0;
pub const SHIP_LIFE: f32 = 100.0;
pub const SHIP_INITIAL_DIRECTION: f32 = 0.0;
pub const SHIP_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0);
pub const SHIP_THRUST: f32 = 5000.0;
pub const SHIP_TURN_THRUST: f32 = 30.0 * TAU;
pub const SHIP_MAX_SPEED: f32 = 500.0;
pub const SHIP_MAX_ANGLE_SPEED: f32 = 1.0 * TAU;
pub const SHIP_RELOAD_TIME: f32 = 0.2;

// Bullet constants
pub const BULLET_SIZE: f32 = 60.0;
pub const BULLET_MASS: f32 = 1.0;
pub const BULLET_LIFE: f32 = 0.7;
pub const BULLET_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);
pub const BULLET_SPEED: f32 = 1000.0;
pub const BULLET_DAMAGE: f32 = 25.0;

// Rock constants
pub const ROCK_RADIUS_MIN: f32 = 20.0;
pub const ROCK_RADIUS_MAX: f32 = 30.0;
pub const ROCK_NB_EDGES: u32 = 18; //3, 4, 7, 11, 18
pub const ROCK_MASS: f32 = 500.0;
pub const ROCK_LIFE: f32 = 100.0;
pub const ROCK_COLOR: Color = Color::new(0.0, 1.0, 1.0, 1.0);
pub const ROCK_MAX_SPEED: f32 = 100.0;
pub const ROCK_MAX_ANGLE_SPEED: f32 = 1.0 * TAU;
pub const ROCK_DAMAGE: f32 = 20.0;

// Star constants
pub const STAR_NUMBER: usize = 100;
pub const STAR_RADIUS_MIN: f32 = 0.5;
pub const STAR_RADIUS_MAX: f32 = 1.5;
pub const STAR_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
