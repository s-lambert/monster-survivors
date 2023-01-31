use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Velocity(Vec2);

fn global_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spritesheet_handle = asset_server.load("player-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(spritesheet_handle, Vec2::new(24.0, 24.0), 4, 1, None, None);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Breakout!".to_string(),
                        width: 500.0,
                        height: 500.0,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_startup_system(global_setup)
        .add_startup_system(setup_player)
        .add_system(animate_player)
        .add_system(bevy::window::close_on_esc)
        .run();
}
