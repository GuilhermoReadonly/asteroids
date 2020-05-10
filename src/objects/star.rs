use crate::{constants::*, objects::*, *};

use ggez::{
    graphics::DrawMode,
    graphics::{Mesh, MeshBuilder},
    Context,
};
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Star {
    pub name: String,
    position: Point,
    direction: Direction,
    mesh: Mesh,
}

impl Star {
    pub fn new(ctx: &mut Context) -> Star {
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

        Self {
            name: "A lone star".to_string(),
            position: Point::new(
                rng.gen_range(-GAME_MAX_WIDTH, GAME_MAX_WIDTH),
                rng.gen_range(-GAME_MAX_HEIGHT, GAME_MAX_HEIGHT),
            ),
            mesh: mesh,
            direction: 0.0,
        }
    }
}

impl Position for Star {
    fn get_position(&self) -> &Point {
        &self.position
    }
    fn get_position_mut(&mut self) -> &mut Point {
        &mut self.position
    }
    fn set_position(&mut self, position: Point) {
        self.position = position
    }

    fn get_direction(&self) -> &Direction {
        &self.direction
    }
    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
}

impl Drawable for Star {
    fn get_mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh
    }
}
