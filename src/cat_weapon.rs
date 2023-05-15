use bevy::prelude::*;

use crate::Player;

pub struct AddCatWeaponEvent;

#[derive(Component)]
struct CatWeapon;

fn spawn_cat_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut add_cat_weapon_reader: EventReader<AddCatWeaponEvent>,
    player_transform_query: Query<&Transform, With<Player>>,
    mut has_spawned: Local<bool>,
) {
    if *has_spawned {
        return;
    }

    if add_cat_weapon_reader.iter().next().is_none() {
        return;
    }

    let Some(player_transform) = player_transform_query.iter().next() else { return };

    *has_spawned = true;
    let spritesheet_handle = asset_server.load("cat.png");
    let texture_atlas =
        TextureAtlas::from_grid(spritesheet_handle, Vec2::new(32.0, 32.0), 8, 4, None, None);
    commands.spawn((
        CatWeapon,
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(27),
            texture_atlas: texture_atlases.add(texture_atlas),
            transform: Transform {
                translation: player_transform.translation,
                ..default()
            },
            ..default()
        },
    ));
}

pub struct CatWeaponPlugin;

impl Plugin for CatWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddCatWeaponEvent>()
            .add_system(spawn_cat_weapon);
    }
}
