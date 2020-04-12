use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH, SHIP_SIZE_X, SHIP_SIZE_Y};
use ggez::nalgebra::Point2;

#[derive(Debug)]
pub struct Ship {
    pub position: Point2<f32>,
    pub perimeter: [Point2<f32>; 4],
}

impl Ship {
    pub fn new() -> Ship {
        let position_x = ARENA_WIDTH / 2.0;
        let position_y = ARENA_HEIGHT / 2.0;

        Ship {
            position: Point2::new(position_x, position_y),
            perimeter: [
                Point2::new(position_x - SHIP_SIZE_X, position_y - SHIP_SIZE_Y),
                Point2::new(position_x, position_y + SHIP_SIZE_Y),
                Point2::new(position_x + SHIP_SIZE_X, position_y - SHIP_SIZE_Y),
                Point2::new(position_x, position_y),
            ],
        }
    }
}
