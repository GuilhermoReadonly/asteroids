use ggez::graphics::Color;
use std::f32::consts::PI;

/// # Game constants

/// The full circle constant (τ)
pub const TAU: f32 = 2.0 * PI;

/// ## Game constants
/// Name of the game
pub const GAME_NAME: &str = "⭐ Asteroids 🚀";
/// Name of the author
pub const GAME_AUTHOR: &str = "718";
/// Frames rate in frames.s⁻¹
pub const GAME_FPS: u32 = 120;
/// ARENA_HEIGHT in px
pub const ARENA_HEIGHT: f32 = 800.0;
/// ARENA_WIDTH in px
pub const ARENA_WIDTH: f32 = 800.0;
/// GAME_MAX_HEIGHT in px
pub const GAME_MAX_HEIGHT: f32 = ARENA_HEIGHT / 2.0;
/// GAME_MAX_WIDTH in px
pub const GAME_MAX_WIDTH: f32 = ARENA_WIDTH / 2.0;
/// SHIP_LINE_WIDTH in px
pub const GAME_LINE_WIDTH: f32 = 1.0;
/// GAME_SHOOT_TIME in s
pub const GAME_SHOOT_TIME: f32 = 0.2;

/// ## Ship constants
pub const SHIP_SIZE_X: f32 = 7.0;
pub const SHIP_SIZE_Y: f32 = 7.0;
pub const SHIP_MASS: f32 = 10.0;
pub const SHIP_LIFE: f32 = 100.0;
pub const SHIP_INITIAL_DIRECTION: f32 = 0.0;
pub const SHIP_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0);
pub const SHIP_THRUST: f32 = 10.0;
pub const SHIP_TURN_INCREMENT: f32 = 0.08 * TAU;
pub const SHIP_MAX_SPEED: f32 = 500.0;

/// ## Bullet constants
pub const BULLET_SIZE_X: f32 = 0.5;
pub const BULLET_SIZE_Y: f32 = 3.0;
pub const BULLET_MASS: f32 = 1.0;
pub const BULLET_LIFE: f32 = 1.5;
pub const BULLET_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);
pub const BULLET_SPEED: f32 = 500.0;
