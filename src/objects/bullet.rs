use crate::{
    constants::*,
    objects::{hit_box::HitBox, Direction, Moveable, Object, Point},
};
use ggez::{graphics::MeshBuilder, Context};

pub type Bullet = Object;

impl Bullet {
    pub fn new_bullet(ctx: &mut Context, position: Point, direction: Direction) -> Bullet {
        let mesh = MeshBuilder::default()
            .line(
                &[Point::new(0.0, 0.0), Point::new(0.0, -BULLET_SIZE)],
                GAME_LINE_WIDTH,
                BULLET_COLOR,
            )
            .unwrap()
            .to_owned()
            .build(ctx)
            .unwrap();

        Self::new(
            "I'm a freaking bullet".to_string(),
            position,
            mesh,
            Self::vec_from_angle(direction) * BULLET_SPEED,
            BULLET_SPEED,
            direction,
            0.0,
            0.0,
            BULLET_MASS,
            BULLET_LIFE,
            HitBox::new(1.0, 1.0),
        )
    }

    pub fn update_life(&mut self, time: f32) {
        self.life -= time;
    }
}
