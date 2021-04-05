use std::error::Error;

use bevy::prelude::*;
use components::{Ball, Collider, Paddle, Scoreboard};
use systems::{
    ball_collision_system, ball_movement_system, paddle_movement_system, scoreboard_system,
};

mod components;
mod systems;

pub fn run() -> Result<(), Box<dyn Error>> {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(Scoreboard { score: 0 })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(paddle_movement_system.system())
        .add_system(ball_collision_system.system())
        .add_system(ball_movement_system.system())
        .add_system(scoreboard_system.system())
        .run();

    Ok(())
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    info!("Setup all we need to play...");
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        // paddle
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .with(Paddle { speed: 500.0 })
        .with(Collider::Paddle)
        // ball
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        })
        // scoreboard
        .spawn(TextBundle {
            text: Text {
                font: asset_server.load("fonts/DroidSans.ttf"),
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.5, 0.5, 1.0),
                    font_size: 40.0,
                    ..Default::default()
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });

    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    commands
        // left
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        // right
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        // bottom
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        // top
        .spawn(SpriteBundle {
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(0.0, bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid);

    // Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec2::new(150.0, 30.0);
    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            commands
                // brick
                .spawn(SpriteBundle {
                    material: brick_material.clone(),
                    sprite: Sprite::new(brick_size),
                    transform: Transform::from_translation(brick_position),
                    ..Default::default()
                })
                .with(Collider::Scorable);
        }
    }

    info!("Setup ready !!!");
}
