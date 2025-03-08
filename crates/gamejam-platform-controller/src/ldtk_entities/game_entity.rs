use crate::game_entities::file_formats::game_entity_definitions::{
    AnimationDescription, GameEntityDefinitionFile, GameEntityDefinitionFileHandle,
};
use crate::graphics::sprite_collection::SpriteCollection;
use crate::ldtk_entities::get_ldtk_string_field;
use crate::player_components::Player;
use crate::scripting::scripted_game_entity::{create_entity_script, EntityScript};
use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use bevy_wasmer_scripting::scripted_entity::WasmEngine;
use bevy_wasmer_scripting::wasm_script_asset::WasmScriptModuleBytes;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::time::Duration;

pub fn game_entity_try_from_entity_instance(
    entity_db: &Res<Assets<GameEntityDefinitionFile>>,
    entity_db_handle: &Res<GameEntityDefinitionFileHandle>,
    entity_instance: &EntityInstance,
    engine: &Res<WasmEngine>,
    asset_server: &Res<AssetServer>,
    wasm_scripts: &mut ResMut<Assets<WasmScriptModuleBytes>>,
    mut transform: Transform,
) -> Option<(impl Bundle, Option<EntityScript>)> {
    if entity_instance.identifier != "game_entity" {
        return None;
    }

    let db = entity_db
        .get(&entity_db_handle.0)
        .expect("missing entity db file");

    let prototype_name = get_ldtk_string_field("prototype_name", &entity_instance)
        .expect("missing prototype_name for game entity");

    let prototype = db
        .entities
        .get(&prototype_name)
        .expect(&format!("missing entity prototype {prototype_name}"));

    let script = prototype
        .script_path
        .as_ref()
        .map(|path| create_entity_script(path, &engine, &asset_server, wasm_scripts.as_mut()));

    transform.scale = Vec3::splat(1.);

    Some((
        (
            transform,
            GameEntity {
                idle_animation: prototype
                    .idle_animation
                    .as_ref()
                    .expect("missing idle animation")
                    .clone(),
                current_animation: prototype
                    .idle_animation
                    .as_ref()
                    .expect("missing idle animation")
                    .animation_name
                    .clone(),
            },
            PlayerDistanceAnimations {
                distance_animations: prototype
                    .distance_based_animations
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.parse::<u32>().unwrap(), v.clone()))
                    .collect(),
            },
        ),
        script,
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
    pub distance_animations: BTreeMap<u32, AnimationDescription>,
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

        for (distance, anim) in distance_animations.distance_animations.iter() {
            if (distance_to_player as u32) > *distance {
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
