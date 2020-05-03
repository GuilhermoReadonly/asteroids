use crate::{
    constants::*,
    objects::*,
};
use ggez::{graphics, graphics::{MeshBuilder, Mesh}, Context};


#[derive(Debug, Clone)]
pub struct Ship {
    pub name: String,
    position: Point,
    mesh: Mesh,
    speed: SpeedVector,
    max_speed: SpeedScalar,
    direction: Direction,
    angle_speed: SpeedAngle,
    max_angle_speed: SpeedAngle,
    mass: Mass,
    pub life: Life,
    hitbox: HitBox,
}

impl Ship {
    pub fn new_ship(ctx: &mut Context) -> Ship {
        let mesh = MeshBuilder::default()
            .polygon(
                graphics::DrawMode::fill(),
                &[
                    Point::new(SHIP_SIZE_X, SHIP_SIZE_Y),
                    Point::new(0.0, -SHIP_SIZE_Y),
                    Point::new(-SHIP_SIZE_X, SHIP_SIZE_Y),
                    Point::new(0.0, 0.0),
                ],
                SHIP_COLOR,
            )
            .unwrap()
            .to_owned()
            .build(ctx)
            .unwrap();

        Self::new(
            "Ship of the Captain".to_string(),
            Point::new(0.0, 0.0),
            mesh,
            SpeedVector::new(0.0, 0.0),
            SHIP_MAX_SPEED,
            SHIP_INITIAL_DIRECTION,
            0.0,
            SHIP_MAX_ANGLE_SPEED,
            SHIP_MASS,
            SHIP_LIFE,
            HitBox::new(2.0 * SHIP_SIZE_X, 2.0 * SHIP_SIZE_Y),
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


impl Position for Ship {
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

impl Speed for Ship {
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

impl Playable for Ship {}

impl Collideable for Ship {
    fn get_hitbox(&self) -> &HitBox {
        &self.hitbox
    }
    fn set_hitbox(&mut self, hitbox: HitBox) {
        self.hitbox = hitbox
    }
}

impl Drawable for Ship{
    fn get_mesh(&self) -> &Mesh { &self.mesh}
    fn set_mesh(&mut self, mesh: Mesh) { self.mesh = mesh }
}

impl Liveable for Ship{
    fn get_life(&self) -> &Life { &self.life}
    fn set_life(&mut self, life: Life) { self.life = life }
}


