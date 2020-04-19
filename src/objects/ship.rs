use crate::{
    constants::*,
    objects::{Object, Point, SpeedVector},
};
use ggez::{graphics, graphics::MeshBuilder, Context};

pub type Ship = Object;

impl Ship {
    pub fn new_ship(ctx: &mut Context) -> Ship {
        let mesh = MeshBuilder::default()
            .polygon(
                graphics::DrawMode::stroke(GAME_LINE_WIDTH),
                &[
                    Point::new(SHIP_SIZE_X, SHIP_SIZE_Y),
                    Point::new(0.0, -SHIP_SIZE_Y),
                    Point::new(-SHIP_SIZE_X, SHIP_SIZE_Y),
                    Point::new(0.0, 0.0),
                ],
                SHIP_COLOR,
            )
            .unwrap()
            .build(ctx)
            .unwrap();

        Self::new(
            "Ship of the Captain".to_string(),
            Point::new(0.0, 0.0),
            mesh,
            SpeedVector::new(0.0, 0.0),
            SHIP_MAX_SPEED,
            SHIP_INITIAL_DIRECTION,
            SHIP_MASS,
            SHIP_LIFE,
            SHIP_COLOR,
        )
    }
}
