use crate::{
    constants::*,
    objects::{Object, Position},
};
use log::info;

pub type Ship = Object;

impl Ship {
    pub fn new_ship() -> Ship {
        let position_x = ARENA_WIDTH / 2.0;
        let position_y = ARENA_HEIGHT / 2.0;

        Self::new(
            "Ship of the Captain".to_string(),
            Position::new(position_x, position_y),
            vec![
                Position::new(-SHIP_SIZE_X, -SHIP_SIZE_Y),
                Position::new(0.0, SHIP_SIZE_Y),
                Position::new(SHIP_SIZE_X, -SHIP_SIZE_Y),
                Position::new(0.0, 0.0),
            ],
            SHIP_INITIAL_SPEED,
            SHIP_INITIAL_DIRECTION,
            SHIP_MASS,
            SHIP_LIFE,
        )
    }

    pub fn shoot(&self) {
        info!("Shoot the mofo !!!");
    }
}
