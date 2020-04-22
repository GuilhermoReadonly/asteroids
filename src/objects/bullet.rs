use crate::{
    constants::*,
    objects::{vec_from_angle, Direction, Object, Point},
};
use ggez::{graphics::MeshBuilder, Context};

pub type Bullet = Object;

impl Bullet {
    pub fn new_bullet(ctx: &mut Context, position: Point, direction: Direction) -> Bullet {
        let mut mesh = MeshBuilder::default().line(
            &[Point::new(0.0, 0.0), Point::new(0.0, -BULLET_SIZE)],
            1.0,
            BULLET_COLOR,
        )
        .unwrap()
        .to_owned();
        let radius = 1.0;
        mesh = Self::show_hit_box(mesh, radius);
        let mesh = mesh.build(ctx).unwrap();

        Self::new(
            "I'm a freaking bullet".to_string(),
            position,
            mesh,
            vec_from_angle(direction) * BULLET_SPEED,
            BULLET_SPEED,
            direction,
            0.0,
            0.0,
            BULLET_MASS,
            BULLET_LIFE,
            radius,
        )
    }

    pub fn update_life(&mut self, time: f32) {
        self.life -= time;
    }
}
