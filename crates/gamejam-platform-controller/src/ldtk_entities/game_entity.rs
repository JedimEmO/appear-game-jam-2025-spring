use std::cmp::Ordering;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::ldtk_entities::{
    get_ldtk_enum_field, get_ldtk_integer_field, get_ldtk_string_array_field, get_ldtk_string_field,
};
use crate::player_components::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use serde::Deserialize;
use std::time::Duration;

pub fn game_entity_try_from_entity_instance(
    entity_instance: &EntityInstance,
    mut transform: Transform
) -> Option<impl Bundle> {
    if entity_instance.identifier != "game_entity" {
        return None;
    }

    let idle_duration = get_ldtk_integer_field("idle_duration", &entity_instance)
        .expect("missing idle_duration for rubble");
    let sprite_name = get_ldtk_string_field("idle_sprite", &entity_instance)
        .expect("missing sprite_name for rubble");

    let animation_name = get_ldtk_string_field("idle_animation", &entity_instance)
        .expect("missing idle_animation for rubble");

    let json = get_ldtk_string_field("distance_animations", &entity_instance)?;
    let distance_animations: PlayerDistanceAnimations = serde_json::from_str(&json).unwrap();

    transform.scale = Vec3::splat(1.);

    Some((
        transform,
        GameEntity {
            idle_animation: AnimationDescription {
                distance: 0,
                sprite_name,
                animation_name: animation_name.clone(),
                duration_millis: idle_duration as u64,
            },
            current_animation: "".to_string(),
        },
        distance_animations,
    ))
}

/// General purpose game entity
/// Can be interactable, attackable and collidable
///
/// Can have different animations depending on distance to player (the closest animation will always play)
#[derive(Component)]
pub struct GameEntity {
    pub idle_animation: AnimationDescription,
    pub current_animation: String,
}

#[derive(Component, Deserialize)]
pub struct PlayerDistanceAnimations {
    pub distance_animations: Vec<AnimationDescription>,
}

#[derive(Deserialize)]
pub struct AnimationDescription {
    pub distance: u32,
    pub sprite_name: String,
    pub animation_name: String,
    pub duration_millis: u64,
}

pub fn player_distance_animation(
    mut commands: Commands,
    animation: Res<SpriteCollection>,
    player_query: Query<&Transform, With<Player>>,
    mut entity_query: Query<
        (
            Entity,
            &Transform,
            &mut GameEntity,
            &PlayerDistanceAnimations,
        ),
        Without<Player>,
    >,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (entity, transform, mut game_entity, distance_animations) in entity_query.iter_mut() {
        let distance_to_player = transform.translation.distance(player_transform.translation);

        let mut min_matched_distance: Option<&AnimationDescription> = None;

        for anim in distance_animations.distance_animations.iter() {
            if (distance_to_player as u32) > anim.distance {
                continue;
            }

            min_matched_distance = Some(anim);
        }

        let sprite_bundle = if let Some(min) = min_matched_distance {
            if game_entity.current_animation == min.animation_name {
                continue;
            }

            game_entity.current_animation = min.animation_name.clone();

            animation
                .create_sprite_animation_bundle(
                    &min.sprite_name,
                    &min.animation_name,
                    Duration::from_millis(min.duration_millis),
                    true,
                    false,
                    false,
                )
                .unwrap()
        } else {
            if game_entity.current_animation == game_entity.idle_animation.sprite_name {
                continue;
            }

            game_entity.current_animation = game_entity.idle_animation.sprite_name.clone();

            animation
                .create_sprite_animation_bundle(
                    &game_entity.idle_animation.sprite_name,
                    &game_entity.idle_animation.animation_name,
                    Duration::from_millis(game_entity.idle_animation.duration_millis),
                    true,
                    false,
                    false,
                )
                .unwrap()
        };

        commands.entity(entity).insert(sprite_bundle);
    }
}
