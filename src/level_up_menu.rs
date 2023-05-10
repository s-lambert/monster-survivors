use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::{cat_weapon::AddCatWeaponEvent, GameState};

#[derive(Component)]
pub struct LevelUpMenu;

#[derive(Component, Debug)]
pub struct ItemChoice {
    id: i32,
}

pub fn add_level_up_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            LevelUpMenu,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(80.0),
                    },
                    margin: UiRect {
                        top: Val::Px(60.0),
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|top_level| {
            top_level
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Px(40.0),
                        },
                        margin: UiRect {
                            bottom: Val::Px(16.0),
                            ..default()
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 1.00).into(),
                    ..default()
                })
                .with_children(|node| {
                    node.spawn(TextBundle::from_section(
                        "Level Up!",
                        TextStyle {
                            font: asset_server.load("pixel_font.ttf"),
                            ..default()
                        },
                    ));
                });
            top_level
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size {
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        flex_grow: 1.0,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|items_container| {
                    items_container
                        .spawn((
                            ItemChoice { id: 1 },
                            ButtonBundle {
                                style: Style {
                                    size: Size {
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 1.00).into(),
                                ..default()
                            },
                        ))
                        .with_children(|item_choice| {
                            item_choice.spawn(TextBundle::from_section(
                                "Item Choice 1",
                                TextStyle {
                                    font: asset_server.load("pixel_font.ttf"),
                                    ..default()
                                },
                            ));
                        });
                    items_container
                        .spawn((
                            ItemChoice { id: 2 },
                            ButtonBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Auto,
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    margin: UiRect {
                                        left: Val::Px(10.0),
                                        right: Val::Px(10.0),
                                        ..default()
                                    },
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 1.00).into(),
                                ..default()
                            },
                        ))
                        .with_children(|item_choice| {
                            item_choice.spawn(TextBundle::from_section(
                                "Item Choice 2",
                                TextStyle {
                                    font: asset_server.load("pixel_font.ttf"),
                                    ..default()
                                },
                            ));
                        });
                    items_container
                        .spawn((
                            ItemChoice { id: 3 },
                            ButtonBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Auto,
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 1.00).into(),
                                ..default()
                            },
                        ))
                        .with_children(|item_choice| {
                            item_choice.spawn(TextBundle::from_section(
                                "Item Choice 3",
                                TextStyle {
                                    font: asset_server.load("pixel_font.ttf"),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

pub fn remove_level_up_menu(mut commands: Commands, menu_query: Query<Entity, With<LevelUpMenu>>) {
    let Some(menu_entity) = menu_query.iter().next() else { return };
    commands.entity(menu_entity).despawn_recursive();
}

pub fn handle_choice(
    interaction_query: Query<(&Interaction, &ItemChoice), Changed<Interaction>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut add_cat_weapon_event: EventWriter<AddCatWeaponEvent>,
) {
    for (interaction, item_choice) in &interaction_query {
        if let Interaction::Clicked = interaction {
            dbg!(interaction);
            dbg!(item_choice);
            rapier_config.physics_pipeline_active = true;
            state.set(GameState::Playing);
            keyboard_input.reset(KeyCode::Space);
            add_cat_weapon_event.send(AddCatWeaponEvent);
        }
    }
}
