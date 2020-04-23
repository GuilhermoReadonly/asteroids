use crate::{
    constants::*,
    objects::{hit_box::HitBox, Object, Point, SpeedVector},
};
use ggez::{graphics, graphics::MeshBuilder, Context};
use rand::prelude::*;

pub type Rock = Object;

impl Rock {
    pub fn new_rock(ctx: &mut Context) -> Rock {
        let mut rng = rand::thread_rng();

        let angle_between_edges = TAU / ROCK_NB_EDGES as f32;

        let mut points: Vec<Point> = vec![];
        for i in 0..ROCK_NB_EDGES {
            let radius: f32 = rng.gen_range(ROCK_RADIUS_MIN, ROCK_RADIUS_MAX);
            let x: f32 = (i as f32 * angle_between_edges).cos() * radius;
            let y: f32 = (i as f32 * angle_between_edges).sin() * radius;
            let point = Point::new(x, y);
            points.push(point);
        }

        let mesh = MeshBuilder::default()
            .polygon(graphics::DrawMode::fill(), &points[..], ROCK_COLOR)
            .unwrap()
            .to_owned()
            .build(ctx)
            .unwrap();

        let position = Point::new(
            rng.gen_range(-GAME_MAX_WIDTH, GAME_MAX_WIDTH),
            rng.gen_range(-GAME_MAX_HEIGHT, GAME_MAX_HEIGHT),
        );

        Self::new(
            "A mofo asteroid".to_string(),
            position,
            mesh,
            SpeedVector::new(
                rng.gen_range(-ROCK_MAX_SPEED, ROCK_MAX_SPEED),
                rng.gen_range(-ROCK_MAX_SPEED, ROCK_MAX_SPEED),
            ),
            // SpeedVector::new(0.0, 0.0),
            ROCK_MAX_SPEED,
            0.0,
            rng.gen_range(-ROCK_MAX_ANGLE_SPEED, ROCK_MAX_ANGLE_SPEED),
            ROCK_MAX_ANGLE_SPEED,
            ROCK_MASS,
            ROCK_LIFE,
            HitBox::new(
                ROCK_RADIUS_MIN + ROCK_RADIUS_MAX,
                ROCK_RADIUS_MIN + ROCK_RADIUS_MAX,
            ),
        )
    }
}
