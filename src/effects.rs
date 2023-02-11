use crate::utils::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct DamageNumberEvent {
    pub dmg: i32,
    pub position: Vec3,
}

#[derive(Component)]
pub struct DamageNumber {
    move_towards: Vec2,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ActiveDamageEffects(pub HashMap<Entity, f32>);

const EFFECT_Z_LAYER: f32 = 99.9;

pub fn display_damage_numbers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut damage_number_reader: EventReader<DamageNumberEvent>,
    mut active_damage_effects: ResMut<ActiveDamageEffects>,
) {
    for damage_number_event in damage_number_reader.iter() {
        let text_style = TextStyle {
            font: asset_server.load("pixel_font.ttf"),
            ..default()
        };
        let position = Vec2::new(
            damage_number_event.position.x,
            damage_number_event.position.y,
        );
        let damage_number_entity = commands
            .spawn((
                DamageNumber {
                    move_towards: Vec2::new(position.x + 2.0, position.y + 20.0),
                },
                Text2dBundle {
                    text: Text::from_section(
                        damage_number_event.dmg.to_string(),
                        text_style.clone(),
                    )
                    .with_alignment(TextAlignment::CENTER),
                    transform: Transform::from_translation(position.extend(EFFECT_Z_LAYER)),
                    ..default()
                },
            ))
            .id();
        active_damage_effects.insert(damage_number_entity, 0.4);
    }
}

pub fn animate_damage_numbers(mut transform_query: Query<(&mut Transform, &DamageNumber)>) {
    for (mut transform, damage_number) in transform_query.iter_mut() {
        transform.translation.x = lerp(transform.translation.x, damage_number.move_towards.x, 0.3);
        transform.translation.y = lerp(transform.translation.y, damage_number.move_towards.y, 0.3);
    }
}

pub fn remove_damage_numbers(
    time: Res<Time>,
    mut commands: Commands,
    mut active_damage_efects: ResMut<ActiveDamageEffects>,
) {
    let delta_seconds = time.delta_seconds();
    active_damage_efects
        .drain_filter(|_k, v| {
            *v -= delta_seconds;
            *v <= 0.0
        })
        .for_each(|(key, ..)| {
            commands.entity(key).despawn();
        });
}
