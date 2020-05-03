use crate::{
    constants::*,
    objects::*,
};
use ggez::{graphics::MeshBuilder, Context};


#[derive(Debug, Clone)]
pub struct Bullet {
    pub name: String,
    position: Point,
    mesh: Mesh,
    speed: SpeedVector,
    max_speed: SpeedScalar,
    direction: Direction,
    angle_speed: SpeedAngle,
    max_angle_speed: SpeedAngle,
    mass: Mass,
    life: Life,
    hitbox: HitBox,
}

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

    pub fn new(
        name: String,
        position: Point,
        mesh: Mesh,
        speed: SpeedVector,
        max_speed: SpeedScalar,
        direction: Direction,
        angle_speed: SpeedAngle,
        max_angle_speed: SpeedAngle,
        mass: Mass,
        life: Life,
        hitbox: HitBox,
    ) -> Self {
        Self {
            name,
            position,
            mesh: mesh,
            speed,
            max_speed,
            direction,
            angle_speed,
            max_angle_speed,
            mass,
            life,
            hitbox,
        }
    }

    pub fn explode(&mut self) {
        info!("KABOOOOOOM !!!!!!!");
    }
}


impl Position for Bullet {
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

impl Speed for Bullet {
    fn get_speed(&self) -> &SpeedVector {
        &self.speed
    }
    fn set_speed(&mut self, speed: SpeedVector) {
        self.speed = speed
    }
    fn get_max_speed(&self) -> &SpeedScalar {
        &self.max_speed
    }
    fn set_max_speed(&mut self, max_speed: SpeedScalar) {
        self.max_speed = max_speed
    }

    fn get_angle_speed(&self) -> &SpeedAngle {
        &self.angle_speed
    }
    fn set_angle_speed(&mut self, angle_speed: SpeedAngle) {
        self.angle_speed = angle_speed
    }
    fn get_max_angle_speed(&self) -> &SpeedAngle {
        &self.max_angle_speed
    }
    fn set_max_angle_speed(&mut self, max_angle_speed: SpeedAngle) {
        self.max_angle_speed = max_angle_speed
    }
    fn get_mass(&self) -> &Mass {
        &self.mass
    }
    fn set_mass(&mut self, mass: Mass) {
        self.mass = mass
    }
}

impl Collideable for Bullet {
    fn get_hitbox(&self) -> &HitBox {
        &self.hitbox
    }
    fn set_hitbox(&mut self, hitbox: HitBox) {
        self.hitbox = hitbox
    }
}

impl Drawable for Bullet{
    fn get_mesh(&self) -> &Mesh { &self.mesh}
    fn set_mesh(&mut self, mesh: Mesh) { self.mesh = mesh }
}

impl Liveable for Bullet{
    fn get_life(&self) -> &Life { &self.life}
    fn set_life(&mut self, life: Life) { self.life = life }
}
