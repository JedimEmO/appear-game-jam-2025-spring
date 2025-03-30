use crate::scripting::scripted_game_entity::{
    EntityScript, GameData, GameEntityHost, GameEntityState,
};
use bevy::prelude::*;
use bevy_wasmer_scripting::scripted_entity::WasmEngine;
use bevy_wasmer_scripting::wasm_script_asset::WasmScriptModuleBytes;
use scripted_game_entity::exports::gamejam::game::entity_resource::StartupSettings;
use scripted_game_entity::gamejam::game::game_host::add_to_linker;
use scripted_game_entity::gamejam::game::game_host::Direction;
use scripted_game_entity::gamejam::game::game_host::EntityUniform;
use scripted_game_entity::GameEntityWorld;
use wasmtime::component::Linker;
use wasmtime::Store;

pub fn create_entity_script(
    entity: Entity,
    script_path: &str,
    engine: &Res<WasmEngine>,
    asset_server: &Res<AssetServer>,
    game_data: &Res<GameData>,
    wasm_scripts: &mut Assets<WasmScriptModuleBytes>,
    script_params: Option<Vec<String>>,
    position: Vec2,
) -> impl Bundle {
    let script: Handle<WasmScriptModuleBytes> = asset_server.load(script_path);
    let script = wasm_scripts.get_mut(&script).unwrap();

    let bytes = script.aot_component_bytes.get_or_insert_with(|| {
        wit_component::ComponentEncoder::default()
            .module(script.wasm_module_bytes.as_slice())
            .unwrap()
            .encode()
            .unwrap()
    });

    let component = wasmtime::component::Component::from_binary(&engine.0, bytes).unwrap();

    let mut store = Store::new(
        &engine.0,
        GameEntityState {
            host: GameEntityHost {
                entity: Entity::PLACEHOLDER,
                queued_commands: vec![],
                game_state: game_data.game_state.clone(),
                player_uniform: EntityUniform {
                    health: None,
                    is_parrying: false,
                    position: (0., 0.),
                    facing: Direction::West,
                },
                self_uniform: EntityUniform {
                    health: None,
                    is_parrying: false,
                    position: (position.x, position.y),
                    facing: Direction::West,
                },
            },
        },
    );

    let mut linker = Linker::<GameEntityState>::new(&engine.0);

    add_to_linker(&mut linker, |state: &mut GameEntityState| &mut state.host).unwrap();

    let settings = StartupSettings {
        params: script_params,
        self_entity_id: entity.to_bits(),
    };

    let entity = GameEntityWorld::instantiate(&mut store, &component, &linker).unwrap();

    let guest = entity.gamejam_game_entity_resource();
    let entity_resource = guest.call_get_entity(&mut store, &settings).unwrap();

    EntityScript {
        game_entity: entity,
        entity_resource,
        store,
    }
}
