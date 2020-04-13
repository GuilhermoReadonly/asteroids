use crate::{
    constants::{ARENA_HEIGHT, ARENA_WIDTH, SHIP_MASS, SHIP_SIZE_X, SHIP_SIZE_Y},
    objects::{Object, Position},
};

pub type Ship = Object;

impl Ship {
    pub fn new_ship() -> Ship {
        let position_x = ARENA_WIDTH / 2.0;
        let position_y = ARENA_HEIGHT / 2.0;

        Self::new(
            Position::new(position_x, position_y),
            vec![
                Position::new(-SHIP_SIZE_X, -SHIP_SIZE_Y),
                Position::new(0.0, SHIP_SIZE_Y),
                Position::new(SHIP_SIZE_X, -SHIP_SIZE_Y),
                Position::new(0.0, 0.0),
            ],
            0.0,
            0.0,
            SHIP_MASS,
            100.0,
        )
    }
}
