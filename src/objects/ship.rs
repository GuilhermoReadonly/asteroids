use crate::{
    constants::*,
    objects::{Object, Point, SpeedVector},
};

pub type Ship = Object;

impl Ship {
    pub fn new_ship() -> Ship {
        Self::new(
            "Ship of the Captain".to_string(),
            Point::new(0.0, 0.0),
            vec![
                Point::new(SHIP_SIZE_X, SHIP_SIZE_Y),
                Point::new(0.0, -SHIP_SIZE_Y),
                Point::new(-SHIP_SIZE_X, SHIP_SIZE_Y),
                Point::new(0.0, 0.0),
            ],
            SpeedVector::new(0.0, 0.0),
            SHIP_MAX_SPEED,
            SHIP_INITIAL_DIRECTION,
            SHIP_MASS,
            SHIP_LIFE,
            SHIP_COLOR,
        )
    }
}
