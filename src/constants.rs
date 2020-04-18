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
/// GAME_FPS in frames s⁻¹
pub const GAME_FPS: u32 = 60;
/// MAX_PHYSICS_VEL in m.s⁻¹
pub const MAX_PHYSICS_VEL: f32 = 250.0;
/// ARENA_HEIGHT in px
pub const ARENA_HEIGHT: f32 = 800.0;
/// ARENA_WIDTH in px
pub const ARENA_WIDTH: f32 = 800.0;

/// ## Ship constants
pub const SHIP_SIZE_X: f32 = 7.0;
pub const SHIP_SIZE_Y: f32 = 7.0;
pub const SHIP_MASS: f32 = 10.0;
pub const SHIP_LIFE: f32 = 100.0;
pub const SHIP_INITIAL_DIRECTION: f32 = 0.0;
pub const SHIP_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0);
pub const SHIP_THRUST: f32 = 10.0;
pub const SHIP_TURN_INCREMENT: f32 = 0.08 * TAU;
pub const SHIP_LINE_WIDTH: f32 = 1.0;

