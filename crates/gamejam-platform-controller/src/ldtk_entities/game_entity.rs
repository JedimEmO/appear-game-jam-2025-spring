use crate::game_entities::file_formats::game_entity_definitions::{
    GameEntityDefinitionFile, GameEntityDefinitionFileHandle,
};
use crate::ldtk_entities::{get_ldtk_string_array_field, get_ldtk_string_field};
use crate::scripting::scripted_game_entity::create_entity_script;
use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use bevy_wasmer_scripting::scripted_entity::WasmEngine;
use bevy_wasmer_scripting::wasm_script_asset::WasmScriptModuleBytes;

pub fn game_entity_try_from_entity_instance(
    entity: Entity,
    entity_db: &Res<Assets<GameEntityDefinitionFile>>,
    entity_db_handle: &Res<GameEntityDefinitionFileHandle>,
    entity_instance: &EntityInstance,
    engine: &Res<WasmEngine>,
    asset_server: &Res<AssetServer>,
    wasm_scripts: &mut ResMut<Assets<WasmScriptModuleBytes>>,
    mut transform: Transform,
) -> Option<(impl Bundle, Option<impl Bundle>)> {
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

    let script_params = get_ldtk_string_array_field("script_params", &entity_instance);

    let script = prototype.script_path.as_ref().map(|path| {
        create_entity_script(
            entity,
            path,
            &engine,
            &asset_server,
            wasm_scripts.as_mut(),
            script_params,
        )
    });

    transform.scale = Vec3::splat(1.);

    Some(((transform, GameEntity {}), script))
}

/// General purpose game entity
/// Can be interactable, attackable and collidable
///
/// Can have different animations depending on distance to player (the closest animation will always play)
#[derive(Component)]
pub struct GameEntity {}
