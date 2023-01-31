use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;

const PLAYER_SPEED: f32 = 100.0;

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
        Player,
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        RigidBody::Dynamic,
        Velocity::default(),
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

// Replace with a spring https://theorangeduck.com/page/spring-roll-call
fn lerp(x: f32, y: f32, t: f32) -> f32 {
    return (1.0 - t) * x + t * y;
}

fn move_player(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let Some(mut player_velocity) = query.iter_mut().next() else { return; };

    if keyboard_input.pressed(KeyCode::Left) {
        player_velocity.linvel.x = lerp(player_velocity.linvel.x, -PLAYER_SPEED, 0.5);
    } else if keyboard_input.pressed(KeyCode::Right) {
        player_velocity.linvel.x = lerp(player_velocity.linvel.x, PLAYER_SPEED, 0.5);
    } else {
        player_velocity.linvel.x = lerp(player_velocity.linvel.x, 0.0, 0.8);
    }
    if keyboard_input.pressed(KeyCode::Down) {
        player_velocity.linvel.y = lerp(player_velocity.linvel.y, -PLAYER_SPEED, 0.5);
    } else if keyboard_input.pressed(KeyCode::Up) {
        player_velocity.linvel.y = lerp(player_velocity.linvel.y, PLAYER_SPEED, 0.5);
    } else {
        player_velocity.linvel.y = lerp(player_velocity.linvel.y, 0.0, 0.8);
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Monster Survivors!".to_string(),
                        width: 500.0,
                        height: 500.0,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(global_setup)
        .add_startup_system(setup_player)
        .add_system(move_player)
        .add_system(animate_player.after(move_player))
        .add_system(bevy::window::close_on_esc)
        .run();
}
