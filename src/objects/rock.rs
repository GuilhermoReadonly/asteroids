use crate::{
    constants::*,
    objects::{Object, Point, SpeedVector},
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
            .polygon(
                graphics::DrawMode::stroke(GAME_LINE_WIDTH),
                &points[..],
                ROCK_COLOR,
            )
            .unwrap()
            .build(ctx)
            .unwrap();

        Self::new(
            "A mofo asteroid".to_string(),
            Point::new(rng.gen_range(-GAME_MAX_WIDTH, GAME_MAX_WIDTH), rng.gen_range(-GAME_MAX_HEIGHT, GAME_MAX_HEIGHT)),
            mesh,
            SpeedVector::new(rng.gen_range(0.0, ROCK_MAX_SPEED), rng.gen_range(0.0, ROCK_MAX_SPEED)),
            ROCK_MAX_SPEED,
            0.0,
            ROCK_MASS,
            ROCK_LIFE,
        )
    }
}
