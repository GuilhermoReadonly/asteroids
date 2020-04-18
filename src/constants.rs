use ggez::graphics::Color;

// Game
pub const GAME_NAME: &str = "Asteroids";
pub const GAME_AUTHOR: &str = "718";
pub const GAME_FPS: u32 = 60;

pub const ARENA_HEIGHT: f32 = 800.0;
pub const ARENA_WIDTH: f32 = 800.0;

pub const SHIP_SIZE_X: f32 = 7.0;
pub const SHIP_SIZE_Y: f32 = 7.0;
pub const SHIP_MASS: f32 = 10.0;
pub const SHIP_LIFE: f32 = 100.0;
pub const SHIP_INITIAL_SPEED: f32 = 0.0;
pub const SHIP_INITIAL_DIRECTION: f32 = 0.0;
pub const SHIP_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0);
pub const SHIP_SPEED_INCREMENT: f32 = 0.3;
pub const SHIP_TURN_INCREMENT: f32 = 1.0;

