use crate::enemies::attackable::Attackable;
use crate::enemies::Enemy;
use crate::ldtk_entities::chest::{
    chest_animation_completed_observer, chest_opening_added_observer, spawn_chest_system, Chest,
    ChestType,
};
use crate::ldtk_entities::interactable::interactable_player_system;
use crate::ldtk_entities::level_transition::{
    level_transition_system, spawn_level_transition_observer, LevelTransition,
};
use crate::ldtk_entities::player_collidable_entity::{player_collidable_system, PlayerCollidable};
use crate::ldtk_entities::player_spawn::{move_player_to_spawn, PlayerSpawnEntity};
use crate::ldtk_entities::rubble::{
    rubble_dead_observer, rubble_dying_observer, spawn_rubble_system, Rubble,
};
use crate::{spawn_terminal_system, spawn_thing_system, GameStates, TerminalBundle, ThingBundle};
use anyhow::anyhow;
use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use bevy_ecs_ldtk::ldtk::{FieldInstance, FieldValue};
use bevy_ecs_ldtk::EntityInstance;
use std::time::Duration;

pub mod chest;
pub mod interactable;
pub mod level_transition;
pub mod player_collidable_entity;
pub mod player_spawn;
pub mod rubble;

pub struct GameLdtkEntitiesPlugin;

impl Plugin for GameLdtkEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_collidable_system,
                spawn_thing_system,
                spawn_terminal_system,
                spawn_chest_system,
                level_transition_system,
                handle_ldtk_entities_spawn,
                move_player_to_spawn
            )
                .run_if(in_state(GameStates::GameLoop)),
        );

        app.add_systems(
            Update,
            (interactable_player_system).run_if(in_state(GameStates::GameLoop)),
        );

        app.add_observer(chest_opening_added_observer)
            .add_observer(spawn_rubble_system)
            .add_observer(rubble_dying_observer)
            .add_observer(rubble_dead_observer)
            .add_observer(spawn_level_transition_observer)
            .add_observer(chest_animation_completed_observer);

        setup_ldtk_entities(app);
    }
}

pub fn handle_ldtk_entities_spawn(
    mut commands: Commands,
    entities: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
) {
    for (entity, entity_instance) in entities.iter() {
        match entity_instance.identifier.as_str() {
            "chest" => {
                let Ok(chest) = Chest::try_from(entity_instance).inspect_err(|e| {
                    error!("failed to extract chest from entity {e}");
                }) else {
                    continue;
                };

                commands.entity(entity).insert(chest);
            }
            "enemy" => {
                info!("Enemy spawned");
                commands.entity(entity).insert(Enemy::default());
            }
            "rubble" => {
                info!("Rubble spawned");
                let collider = get_ldtk_bool_field("collider", &entity_instance)
                    .expect("missing collider for rubble");
                let idle_millis = get_ldtk_integer_field("idle_animation_millis", &entity_instance)
                    .expect("missing idle_animation_millis for rubble");
                let death_animation_millis =
                    get_ldtk_integer_field("death_animation_millis", &entity_instance)
                        .expect("missing death_animation_millis for rubble");
                let dead_animation_millis =
                    get_ldtk_integer_field("dead_animation_millis", &entity_instance)
                        .expect("missing dead_animation_millis for rubble");
                let sprite_name = get_ldtk_enum_field::<String>("rubble_type", &entity_instance)
                    .expect("missing rubble_type for rubble")
                    .expect("");

                commands.entity(entity).insert(Rubble {
                    collider,
                    sprite_name,
                    idle_duration: Duration::from_millis(idle_millis as u64),
                    death_duration: Duration::from_millis(death_animation_millis as u64),
                    dead_duration: Duration::from_millis(dead_animation_millis as u64),
                });
            }
            "level_transition" => {
                info!("level transition spawned");
                let target_level_index =
                    get_ldtk_integer_field("target_level_index", &entity_instance)
                        .expect("missing target_level_index for level transition");
                let target_spawn = get_ldtk_string_field("target_spawn", &entity_instance)
                    .expect("missing target_spawn for level transition");

                commands.entity(entity).insert((
                    LevelTransition {
                        target_level_index,
                        target_player_spawn_name: target_spawn,
                    },
                    PlayerCollidable,
                ));
            }
            "playerspawn" => {
                let spawn_name = get_ldtk_string_field("name", &entity_instance)
                    .expect("missing name for player spawn");

                commands
                    .entity(entity)
                    .insert(PlayerSpawnEntity { spawn_name });
            }
            _ => {
                info!("Attempting to spawn unknown entity {:?}", entity_instance)
            }
        }
    }
}

pub fn setup_ldtk_entities(app: &mut App) {
    app.register_ldtk_entity_for_layer::<ThingBundle>("things", "branch");
    app.register_ldtk_entity_for_layer::<TerminalBundle>("things", "terminal");
}

pub fn get_ldtk_enum_field<T: TryFrom<String>>(
    key: &str,
    entity_instance: &EntityInstance,
) -> anyhow::Result<Option<T>> {
    for field in &entity_instance.field_instances {
        if field.identifier != key {
            continue;
        }

        return Ok(Some(try_from_ldtk_enum_field::<T>(field)?));
    }

    Ok(None)
}

pub fn try_from_ldtk_enum_field<T: TryFrom<String>>(field: &FieldInstance) -> anyhow::Result<T> {
    match &field.value {
        FieldValue::Enum(value) => {
            let Some(value) = value else {
                return Err(anyhow!("no enum value"));
            };

            T::try_from(value.clone())
                .map_err(|_| anyhow!("failed to convert string into enum value {value}"))
        }
        _ => Err(anyhow!("not an enum")),
    }
}

pub fn get_ldtk_bool_field(key: &str, entity_instance: &EntityInstance) -> Option<bool> {
    for field in &entity_instance.field_instances {
        if field.identifier != key {
            continue;
        }

        return match field.value {
            FieldValue::Bool(v) => Some(v),
            _ => None,
        };
    }

    None
}

pub fn get_ldtk_integer_field(key: &str, entity_instance: &EntityInstance) -> Option<i32> {
    for field in &entity_instance.field_instances {
        if field.identifier != key {
            continue;
        }

        return match field.value {
            FieldValue::Int(v) => v,
            _ => None,
        };
    }

    None
}

pub fn get_ldtk_string_field(key: &str, entity_instance: &EntityInstance) -> Option<String> {
    for field in &entity_instance.field_instances {
        if field.identifier != key {
            continue;
        }

        return match &field.value {
            FieldValue::String(v) => v.clone(),
            _ => None,
        };
    }

    None
}
