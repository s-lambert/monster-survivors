use bevy::prelude::*;
use bevy::sprite::*;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player {
    max_hp: i32,
    hp: i32,
}

#[derive(Component)]
struct PlayerHpBar;

#[derive(Resource)]
struct PlayerHitCooldown(HashMap<Entity, f32>);

#[derive(Component)]
struct Enemy;

const PLAYER_SPEED: f32 = 100.0;
const ENEMY_SPEED: f32 = 80.0;

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
    commands
        .spawn((
            Player {
                max_hp: 100,
                hp: 100,
            },
            SpriteSheetBundle {
                texture_atlas: texture_atlases.add(texture_atlas),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            RigidBody::Dynamic,
            Collider::cuboid(8.0, 10.0),
            LockedAxes::ROTATION_LOCKED,
            // Make it so the player stays stationary when colliding with enemies.
            Dominance::group(10),
            Velocity::default(),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(18.0, 2.0, 0.0),
                    translation: Vec3::new(0.0, -14.0, 0.0),
                    ..default()
                },
                ..default()
            });
            parent.spawn((
                PlayerHpBar,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 0.0),
                        anchor: Anchor::CenterLeft,
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::new(18.0, 2.0, 0.0),
                        translation: Vec3::new(-9.0, -14.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn spawn_bat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spritesheet_handle = asset_server.load("bat-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(spritesheet_handle, Vec2::new(24.0, 24.0), 4, 1, None, None);
    commands.spawn((
        Enemy,
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            transform: Transform {
                // TODO: Spawn outside of scene
                translation: Vec3::new(200.0, 200.0, 0.0),
                scale: Vec3::splat(1.5),
                ..default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        RigidBody::Dynamic,
        Collider::cuboid(8.0, 8.0),
        LockedAxes::ROTATION_LOCKED,
        Velocity::default(),
    ));
}

fn animate_loops(
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

// TODO: Replace with a spring https://theorangeduck.com/page/spring-roll-call
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

fn move_towards_player(
    player_transform_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Velocity), Without<Player>>,
) {
    let Some(player_transform) = player_transform_query.iter().next() else { return };
    for (enemy_transform, mut enemy_velocity) in enemy_query.iter_mut() {
        let direction_to_player = (player_transform.translation - enemy_transform.translation)
            .normalize()
            .truncate();
        enemy_velocity.linvel = direction_to_player * ENEMY_SPEED;
    }
}

fn player_enemy_collisions(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut player_hit_cooldown: ResMut<PlayerHitCooldown>,
    mut player_entity_query: Query<(Entity, &mut Player)>,
) {
    let (player_entity, mut player) = player_entity_query.single_mut();

    let delta_seconds = time.delta_seconds();
    player_hit_cooldown
        .0
        .drain_filter(|_k, v| {
            *v -= delta_seconds;
            *v <= 0.0
        })
        .for_each(drop);

    for contact_pair in rapier_context.contacts_with(player_entity) {
        let enemy_collider = if contact_pair.collider1() == player_entity {
            contact_pair.collider2()
        } else {
            contact_pair.collider1()
        };

        if !player_hit_cooldown.0.contains_key(&enemy_collider) {
            player.hp -= 5;
            player_hit_cooldown.0.insert(enemy_collider, 0.5);
        }
    }
}

fn animate_hp_bar(
    player_query: Query<&Player, Changed<Player>>,
    mut bar_transform_query: Query<&mut Transform, With<PlayerHpBar>>,
) {
    let Some(player) = player_query.iter().next() else { return };
    let mut bar_transform = bar_transform_query.single_mut();

    bar_transform.scale.x = (player.hp as f32 / player.max_hp as f32).max(0.0) * 18.0;
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
        .insert_resource(PlayerHitCooldown(HashMap::default()))
        .add_startup_system(global_setup)
        .add_startup_system(setup_player)
        .add_startup_system(spawn_bat)
        .add_system(move_player)
        .add_system(move_towards_player)
        .add_system(animate_loops.after(move_player))
        .add_system(player_enemy_collisions.after(move_player))
        .add_system(animate_hp_bar.after(player_enemy_collisions))
        .add_system(bevy::window::close_on_esc)
        .run();
}
