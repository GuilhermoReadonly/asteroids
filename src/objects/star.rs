use crate::{
    constants::*,
    objects::{Object, Point, SpeedVector},
};
use ggez::{graphics::DrawMode, graphics::MeshBuilder, Context};
use rand::prelude::*;

pub type Star = Object;

impl Star {
    pub fn new_star(ctx: &mut Context) -> Star {
        let mut rng = rand::thread_rng();
        let radius: f32 = rng.gen_range(STAR_RADIUS_MIN, STAR_RADIUS_MAX);
        let mesh = MeshBuilder::default()
            .circle(
                DrawMode::fill(),
                Point::new(0.0, 0.0),
                radius,
                100.0,
                STAR_COLOR,
            )
            .build(ctx)
            .unwrap();

        Self::new(
            "A lone star".to_string(),
            Point::new(
                rng.gen_range(-GAME_MAX_WIDTH, GAME_MAX_WIDTH),
                rng.gen_range(-GAME_MAX_HEIGHT, GAME_MAX_HEIGHT),
            ),
            mesh,
            SpeedVector::new(0.0, 0.0),
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            radius,
        )
    }
}
