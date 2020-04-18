use crate::{
    constants::*,
    objects::{Object, Point, Speed},
};
use log::info;

pub type Ship = Object;

impl Ship {
    pub fn new_ship() -> Ship {
        let position_x = ARENA_WIDTH / 2.0;
        let position_y = ARENA_HEIGHT / 2.0;

        Self::new(
            "Ship of the Captain".to_string(),
            Point::new(position_x, position_y),
            vec![
                Point::new(-SHIP_SIZE_X, -SHIP_SIZE_Y),
                Point::new(0.0, SHIP_SIZE_Y),
                Point::new(SHIP_SIZE_X, -SHIP_SIZE_Y),
                Point::new(0.0, 0.0),
            ],
            Speed::new(0.0,0.0),
            SHIP_INITIAL_DIRECTION,
            SHIP_MASS,
            SHIP_LIFE,
        )
    }

    pub fn shoot(&self) {
        info!("Shoot the mofo !!!");
    }
}
