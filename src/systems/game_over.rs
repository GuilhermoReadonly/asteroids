use bevy::prelude::*;

use crate::{constants::*, AppState, ColorMaterials};

pub fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ColorMaterials>,
) {
    info!("Setup all the game over screen...");
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // ui
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(70.0)),
                border: Rect::all(Val::Px(1.0)),
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            material: button_materials.green.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.black.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Game Over",
                            TextStyle {
                                font: asset_server.load(GAME_FONT),
                                font_size: 40.0,
                                color: GREEN,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });

    info!("Setup ready !!!");
}

pub fn return_to_menu_system(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    button_bg_materials: Res<ColorMaterials>,
    mut entity_query: Query<(Entity,)>,
    mut interaction_query: Query<
        (&mut Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (mut interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = GREEN;
                *material = button_bg_materials.black.clone();

                // Remove entities from screen
                for (entity,) in entity_query.iter_mut() {
                    commands.entity(entity).despawn();
                }

                *interaction = Interaction::None;
                state.set(AppState::Menu).unwrap();
            }
            Interaction::Hovered => {
                text.sections[0].style.color = BLACK;
                *material = button_bg_materials.green.clone();
            }
            Interaction::None => {
                text.sections[0].style.color = GREEN;
                *material = button_bg_materials.black.clone();
            }
        }
    }
}
