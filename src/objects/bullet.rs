use crate::{
    constants::*,
    objects::{vec_from_angle, Direction, Object, Point},
};

pub type Bullet = Object;

impl Bullet {
    pub fn new_bullet(position: Point, direction: Direction) -> Bullet {
        Self::new(
            "I'm a freaking bullet".to_string(),
            position,
            vec![
                Point::new(BULLET_SIZE_X, BULLET_SIZE_Y),
                Point::new(BULLET_SIZE_X, -BULLET_SIZE_Y),
                Point::new(-BULLET_SIZE_X, -BULLET_SIZE_Y),
                Point::new(-BULLET_SIZE_X, BULLET_SIZE_Y),
            ],
            vec_from_angle(direction) * BULLET_SPEED,
            BULLET_SPEED,
            direction,
            BULLET_MASS,
            BULLET_LIFE,
            BULLET_COLOR,
        )
    }

    pub fn update_life(&mut self, dt: f32) {
        self.life -= dt;
    }
}
