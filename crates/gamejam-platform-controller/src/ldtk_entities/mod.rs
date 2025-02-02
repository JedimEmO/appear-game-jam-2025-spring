use crate::ldtk_entities::chest::{chest_animation_completed_observer, chest_opening_added_observer, spawn_chest_system, Chest, ChestType};
use crate::ldtk_entities::interactable::interactable_player_system;
use crate::{
    spawn_terminal_system, spawn_thing_system, GameStates, PlayerSpawnEntityBundle, TerminalBundle,
    ThingBundle,
};
use anyhow::anyhow;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use bevy_ecs_ldtk::ldtk::{FieldInstance, FieldValue};
use bevy_ecs_ldtk::EntityInstance;

pub mod chest;
pub mod interactable;

pub struct GameLdtkEntitiesPlugin;

impl Plugin for GameLdtkEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_thing_system,
                spawn_terminal_system,
                spawn_chest_system,
                handle_ldtk_entities_spawn,
            )
                .run_if(in_state(GameStates::GameLoop)),
        );

        app.add_systems(
            Update,
            (interactable_player_system).run_if(in_state(GameStates::GameLoop)),
        );

        app.add_observer(chest_opening_added_observer);
        app.add_observer(chest_animation_completed_observer);

        setup_ldtk_entities(app);
    }
}

pub fn handle_ldtk_entities_spawn(
    mut commands: Commands,
    entities: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
) {
    for (entity, entity_instance) in entities.iter() {
        match entity_instance.identifier.as_str() {
            "Chest" => {
                let Ok(chest) = Chest::try_from(entity_instance).inspect_err(|e| {
                    error!("failed to extract chest from entity {e}");
                }) else {
                    continue;
                };

                commands.entity(entity).insert(chest);
            }
            _ => {}
        }
    }
}

pub fn setup_ldtk_entities(app: &mut App) {
    app.register_ldtk_entity::<PlayerSpawnEntityBundle>("PlayerSpawn");
    app.register_ldtk_entity_for_layer::<ThingBundle>("Things", "Branch");
    app.register_ldtk_entity_for_layer::<TerminalBundle>("Things", "Terminal");
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
                .map_err(|_| anyhow!("failed to convert string into enum value"))
        }
        _ => Err(anyhow!("not an enum")),
    }
}
