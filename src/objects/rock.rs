use crate::{constants::*, objects::*, *};
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
    nb_edges: u32,
    pub radius: f32,
}

impl Rock {
    pub fn new_init(ctx: &mut Context, nb_edges: u32, radius: f32) -> Rock {
        let mut rng = rand::thread_rng();
        let position = Point::new(
            rng.gen_range(-GAME_MAX_WIDTH, GAME_MAX_WIDTH),
            rng.gen_range(-GAME_MAX_HEIGHT, GAME_MAX_HEIGHT),
        );
        Self::new(ctx, nb_edges, radius, position, ROCK_MASS)
    }

    pub fn new(ctx: &mut Context, nb_edges: u32, radius: f32, position: Point, mass: Mass) -> Rock {
        let mut rng = rand::thread_rng();

        let angle_between_edges = TAU / nb_edges as f32;

        let mut points: Vec<Point> = vec![];
        for i in 0..nb_edges {
            let current_radius: f32 =
                rng.gen_range(radius - ROCK_RADIUS_DELTA, radius + ROCK_RADIUS_DELTA);
            let x: f32 = (i as f32 * angle_between_edges).cos() * current_radius;
            let y: f32 = (i as f32 * angle_between_edges).sin() * current_radius;
            let point = Point::new(x, y);
            points.push(point);
        }

        let mesh = MeshBuilder::default()
            .polygon(graphics::DrawMode::fill(), &points[..], GAME_COLOR_BLACK)
            .unwrap()
            .polygon(
                graphics::DrawMode::stroke(GAME_LINE_WIDTH),
                &points[..],
                ROCK_COLOR,
            )
            .unwrap()
            .to_owned()
            .build(ctx)
            .unwrap();

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
            mass: mass,
            life: ROCK_LIFE,
            hitbox: HitBox::new(2.0 * radius, 2.0 * radius),
            nb_edges: nb_edges,
            radius: radius,
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

impl Breakable for Rock {
    fn break_it(&self, ctx: &mut Context) -> Vec<Self> {
        let position1 = Point::new(self.position.x + self.radius, self.position.y + self.radius);
        let position2 = Point::new(self.position.x - self.radius, self.position.y - self.radius);
        let rock_1 = Self::new(
            ctx,
            self.get_nb_edges() - 1,
            self.radius - ROCK_RADIUS_DECREMENT,
            position1,
            self.mass - ROCK_MASS_DECREMENT,
        );
        let rock_2 = Self::new(
            ctx,
            self.get_nb_edges() - 1,
            self.radius - ROCK_RADIUS_DECREMENT,
            position2,
            self.mass - ROCK_MASS_DECREMENT,
        );
        let result = vec![rock_1, rock_2];
        result
    }

    fn get_nb_edges(&self) -> u32 {
        self.nb_edges
    }
}
