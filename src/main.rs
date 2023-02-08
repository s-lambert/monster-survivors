use bevy::prelude::*;
use bevy::sprite::*;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

mod physics_groups;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum GameState {
    Playing,
    Paused,
}

#[derive(Component, Deref, DerefMut)]
struct PlayerAnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct LoopAnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct FireballTimer(Timer);

#[derive(Component)]
struct Player {
    lvl: i32,
    curr_exp: i32,
    next_exp: i32,
    max_hp: i32,
    hp: i32,
}

#[derive(Component)]
struct PlayerHpBar;

#[derive(Resource, Deref, DerefMut)]
struct PlayerHitCooldown(HashMap<Entity, f32>);

#[derive(Component)]
struct EnemySpawner {
    timer: Timer,
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Attack;

const WINDOW_SIZE: f32 = 500.0;
const PLAYER_SPEED: f32 = 100.0;
const PLAYER_HP_WIDTH: f32 = 18.0;
const FIREBALL_COOLDOWN: f32 = 0.5;
const FIREBALL_SPEED: f32 = 200.0;
const ENEMY_SPEED: f32 = 80.0;
const ENEMY_DAMAGE: i32 = 5;
const ENEMY_DAMAGE_COOLDOWN: f32 = 0.5;
const GEM_EXP: i32 = 40;

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
                lvl: 1,
                curr_exp: 0,
                next_exp: 100,
                max_hp: 100,
                hp: 100,
            },
            SpriteSheetBundle {
                texture_atlas: texture_atlases.add(texture_atlas),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            },
            PlayerAnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            FireballTimer(Timer::from_seconds(FIREBALL_COOLDOWN, TimerMode::Repeating)),
            RigidBody::Dynamic,
            Collider::cuboid(8.0, 10.0),
            CollisionGroups::new(
                physics_groups::PLAYER_GROUP,
                physics_groups::ENEMY_GROUP | physics_groups::PICKUP_GROUP,
            ),
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
                        translation: Vec3::new(-9.0, -14.0, 1.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: This only covers a 3x3 grid, it needs to be endless.
    let world_size = 3;
    let background_size = 1024.0;
    let starting_point = -(background_size * ((world_size - 1) as f32) / 2.0);
    for row in 0..world_size {
        for column in 0..world_size {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("background.png"),
                transform: Transform {
                    translation: Vec3::new(
                        remap(
                            0.0,
                            (world_size - 1) as f32,
                            starting_point,
                            -starting_point,
                            row as f32,
                        ),
                        remap(
                            0.0,
                            (world_size - 1) as f32,
                            starting_point,
                            -starting_point,
                            column as f32,
                        ),
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn setup_spawns(mut commands: Commands) {
    commands.spawn(EnemySpawner {
        timer: Timer::from_seconds(0.4, TimerMode::Repeating),
    });
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut enemy_spawner_query: Query<&mut EnemySpawner>,
    player_transform_query: Query<&Transform, With<Player>>,
) {
    let Some(player_transform) = player_transform_query.iter().next() else { return };
    let mut rng = rand::thread_rng();
    for mut enemy_spawner in &mut enemy_spawner_query {
        enemy_spawner.timer.tick(time.delta());
        if enemy_spawner.timer.just_finished() {
            let distance_from_center: f32 = WINDOW_SIZE / 2.0;
            let radius =
                (distance_from_center.powf(2.0) + distance_from_center.powf(2.0)).sqrt() + 10.0;
            let rotation = rng.gen_range(0.0..PI * 2.0);
            let point_on_circle = Vec2::new(rotation.cos(), rotation.sin());
            let point_around_player =
                player_transform.translation + (point_on_circle * radius).extend(0.0);
            spawn_bat(
                &mut commands,
                &asset_server,
                &mut texture_atlases,
                point_around_player,
            );
        }
    }
}

fn spawn_bat(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    translation: Vec3,
) {
    let spritesheet_handle = asset_server.load("bat-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(spritesheet_handle, Vec2::new(24.0, 24.0), 4, 1, None, None);

    commands.spawn((
        Enemy,
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            transform: Transform {
                translation: translation,
                scale: Vec3::splat(1.5),
                ..default()
            },
            ..default()
        },
        LoopAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        RigidBody::Dynamic,
        Collider::cuboid(8.0, 8.0),
        CollisionGroups::new(
            physics_groups::ENEMY_GROUP,
            physics_groups::ENEMY_GROUP
                | physics_groups::PLAYER_GROUP
                | physics_groups::ATTACK_GROUP,
        ),
        LockedAxes::ROTATION_LOCKED,
        Velocity::default(),
    ));
}

fn animate_loops(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut LoopAnimationTimer,
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

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut PlayerAnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Velocity,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, velocity) in &mut query {
        // 10.0 is an arbitrary number that the velocity will reach while slowing down.
        if velocity.linvel.length() <= 10.0 {
            continue;
        }
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

fn inverse_lerp(x: f32, y: f32, v: f32) -> f32 {
    return (v - x) / (y - x);
}

fn remap(input_min: f32, input_max: f32, output_min: f32, output_max: f32, value: f32) -> f32 {
    let t = inverse_lerp(input_min, input_max, value);
    return lerp(output_min, output_max, t);
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
    mut enemy_query: Query<(&Transform, &mut Velocity), With<Enemy>>,
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
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let (player_entity, mut player) = player_entity_query.single_mut();

    let delta_seconds = time.delta_seconds();
    player_hit_cooldown
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

        // TODO: Could be done another way, maybe filter groups in rapier?
        if enemy_query.get(enemy_collider).is_ok() {
            if !player_hit_cooldown.contains_key(&enemy_collider) {
                player.hp -= ENEMY_DAMAGE;
                player_hit_cooldown
                    .0
                    .insert(enemy_collider, ENEMY_DAMAGE_COOLDOWN);
            }
        }
    }
}

fn attack_enemy_collisions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    rapier_context: Res<RapierContext>,
    attack_query: Query<Entity, With<Attack>>,
    enemy_transform_query: Query<&Transform, With<Enemy>>,
) {
    'attack_loop: for attack_entity in attack_query.iter() {
        for (collider1, collider2, intersecting) in rapier_context.intersections_with(attack_entity)
        {
            if intersecting {
                let enemy_entity = if collider1 == attack_entity {
                    collider2
                } else {
                    collider1
                };
                let Ok(enemy_transform) = enemy_transform_query.get(enemy_entity) else { continue 'attack_loop };
                spawn_gem(&mut commands, &asset_server, enemy_transform.translation);
                commands.entity(collider1).despawn();
                commands.entity(collider2).despawn();
                // Only kill the first enemy that gets hit.
                continue 'attack_loop;
            }
        }
    }
}

fn pickup_gems(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(Entity, &mut Player)>,
) {
    let Some((player_entity, mut player)) = player_query.iter_mut().next() else { return };
    for (collider1, collider2, intersecting) in rapier_context.intersections_with(player_entity) {
        if intersecting {
            let gem_entity = if collider1 == player_entity {
                collider2
            } else {
                collider1
            };
            commands.entity(gem_entity).despawn();
            player.curr_exp += GEM_EXP;
        }
    }
}

fn level_up(mut player_query: Query<&mut Player>) {
    let Some(mut player) = player_query.iter_mut().next() else { return };
    if player.curr_exp >= player.next_exp {
        player.curr_exp -= player.next_exp;
        player.lvl += 1;
        player.next_exp = ((((player.lvl as f32).log(10.0)) + player.lvl as f32) * 100.0) as i32;
        dbg!("Levelled up {} to next level.", player.next_exp);
    }
}

fn spawn_gem(commands: &mut Commands, asset_server: &Res<AssetServer>, enemy_position: Vec3) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("gem.png"),
            transform: Transform {
                translation: enemy_position,
                scale: Vec3::splat(0.6),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Sensor,
        Collider::ball(10.0),
        CollisionGroups::new(physics_groups::PICKUP_GROUP, physics_groups::PLAYER_GROUP),
        Velocity::default(),
    ));
}

fn animate_hp_bar(
    player_query: Query<&Player, Changed<Player>>,
    mut bar_transform_query: Query<&mut Transform, With<PlayerHpBar>>,
) {
    let Some(player) = player_query.iter().next() else { return };
    let mut bar_transform = bar_transform_query.single_mut();

    bar_transform.scale.x = (player.hp as f32 / player.max_hp as f32).max(0.0) * PLAYER_HP_WIDTH;
}

fn launch_fireball(
    commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut timer_query: Query<&mut FireballTimer>,
    player_transform_query: Query<&Transform, With<Player>>,
    enemy_transform_query: Query<&Transform, With<Enemy>>,
) {
    let Some(mut timer) = timer_query.iter_mut().next() else { return };

    timer.tick(time.delta());
    if timer.just_finished() {
        let Some(player_position) = player_transform_query.iter().next().map(|transform| transform.translation.truncate()) else { return };
        let Some(relative_enemy_position) = enemy_transform_query.iter()
            .map(|transform| transform.translation.truncate() - player_position)
            .reduce(|closest_relative_position, relative_position| {
                if relative_position.length() < closest_relative_position.length() {
                    relative_position
                } else {
                    closest_relative_position
                }
            }) else { return };
        let rotation_radians =
            relative_enemy_position.y.atan2(relative_enemy_position.x) + PI / 2.0;

        spawn_fireball(
            commands,
            asset_server,
            player_position.extend(1.0),
            rotation_radians,
            relative_enemy_position.normalize(),
        );
    }
}

fn spawn_fireball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    position: Vec3,
    rotation_radians: f32,
    direction: Vec2,
) {
    commands.spawn((
        Attack,
        SpriteBundle {
            texture: asset_server.load("effects/fireball.png"),
            transform: Transform {
                translation: position,
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation_radians),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Sensor,
        Collider::ball(10.0),
        CollisionGroups::new(physics_groups::ATTACK_GROUP, physics_groups::ENEMY_GROUP),
        Velocity::linear(direction * FIREBALL_SPEED),
    ));
}

fn camera_follow_player(
    mut camera_transform_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_transform_query: Query<&Transform, With<Player>>,
) {
    let Some(mut camera_transform) = camera_transform_query.iter_mut().next() else { return };
    let Some(player_transform) = player_transform_query.iter().next() else { return };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn pause_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        rapier_config.physics_pipeline_active = false;
        state.push(GameState::Paused).unwrap();
        keyboard_input.reset(KeyCode::Space);
    }
}

fn unpause_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        rapier_config.physics_pipeline_active = true;
        state.pop().unwrap();
        keyboard_input.reset(KeyCode::Space);
    }
}

fn main() {
    App::new()
        .add_state(GameState::Playing)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Monster Survivors!".to_string(),
                        width: WINDOW_SIZE,
                        height: WINDOW_SIZE,
                        ..default()
                    },
                    ..default()
                }),
        )
        .insert_resource(RapierConfiguration {
            gravity: Vect::new(0.0, 0.0),
            ..default()
        })
        .insert_resource(PlayerHitCooldown(HashMap::default()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(global_setup)
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_background)
                .with_system(setup_player)
                .with_system(setup_spawns),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(pause_game)
                .with_system(spawn_enemies)
                .with_system(move_player)
                .with_system(move_towards_player)
                .with_system(animate_loops)
                .with_system(animate_player)
                .with_system(launch_fireball)
                .with_system(attack_enemy_collisions)
                .with_system(pickup_gems)
                .with_system(level_up.after(pickup_gems))
                .with_system(player_enemy_collisions.after(attack_enemy_collisions))
                .with_system(animate_hp_bar.after(player_enemy_collisions)),
        )
        .add_system_set(SystemSet::on_update(GameState::Paused).with_system(unpause_game))
        // TOOD: Not sure if this is the right place to add it, see if there's a way to add after a plugin.
        .add_system_to_stage(CoreStage::PostUpdate, camera_follow_player)
        .add_system(bevy::window::close_on_esc)
        .run();
}
