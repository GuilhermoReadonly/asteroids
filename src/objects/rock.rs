use crate::{constants::*, objects::*};
use ggez::{
    graphics,
    graphics::{Mesh, MeshBuilder},
    Context,
};
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Rock {
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

impl Rock {
    pub fn new(ctx: &mut Context) -> Rock {
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

        Self {
            name: "A mofo asteroid".to_string(),
            position: position,
            mesh: mesh,
            speed: SpeedVector::new(
                rng.gen_range(-ROCK_MAX_SPEED, ROCK_MAX_SPEED),
                rng.gen_range(-ROCK_MAX_SPEED, ROCK_MAX_SPEED),
            ),
            // SpeedVector::new(0.0, 0.0),
            max_speed: ROCK_MAX_SPEED,
            direction: 0.0,
            angle_speed: rng.gen_range(-ROCK_MAX_ANGLE_SPEED, ROCK_MAX_ANGLE_SPEED),
            max_angle_speed: ROCK_MAX_ANGLE_SPEED,
            mass: ROCK_MASS,
            life: ROCK_LIFE,
            hitbox: HitBox::new(
                ROCK_RADIUS_MIN + ROCK_RADIUS_MAX,
                ROCK_RADIUS_MIN + ROCK_RADIUS_MAX,
            ),
        }
    }
}

impl Position for Rock {
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

impl Speed for Rock {
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

impl Collideable for Rock {
    fn get_hitbox(&self) -> &HitBox {
        &self.hitbox
    }
    fn set_hitbox(&mut self, hitbox: HitBox) {
        self.hitbox = hitbox
    }
}

impl Drawable for Rock {
    fn get_mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh
    }
}

impl Liveable for Rock {
    fn get_life(&self) -> &Life {
        &self.life
    }
    fn set_life(&mut self, life: Life) {
        self.life = life
    }
}
