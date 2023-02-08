use bevy::prelude::*;

#[derive(Component)]
pub struct LevelUpMenu;

#[derive(Component)]
pub struct ItemChoice;

pub fn add_level_up_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            LevelUpMenu,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect {
                        top: Val::Px(100.0),
                        ..default()
                    },
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                    },
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
                            height: Val::Px(40.0),
                            width: Val::Percent(100.0),
                        },
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
                            height: Val::Auto,
                            width: Val::Percent(100.0),
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|items_container| {
                    items_container
                        .spawn((
                            ItemChoice,
                            ButtonBundle {
                                style: Style {
                                    size: Size {
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
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
                            ItemChoice,
                            ButtonBundle {
                                style: Style {
                                    size: Size {
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
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
                            ItemChoice,
                            ButtonBundle {
                                style: Style {
                                    size: Size {
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
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

pub fn handle_choice() {
    // TODO: Add choice detection + player updates.
}
