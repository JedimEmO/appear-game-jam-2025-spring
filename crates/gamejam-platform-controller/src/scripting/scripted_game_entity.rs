
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::reflect::ReflectCommandExt;
use bevy::log::info;
use bevy::prelude::{Commands, Component, Entity, Query, Res};
use bevy::reflect::TypePath;
use bevy_wasmer_scripting::scripted_entity::WasmEngine;
use bevy_wasmer_scripting::wasm_script_asset::WasmScriptModuleBytes;
use wasmtime::component::Linker;
use wasmtime::{AsContextMut, Store};
use gamejam_bevy_components::Interactable;
use crate::scripting::game_entity::GameEntity;
use crate::scripting::game_entity::gamejam::game::game_host::{add_to_linker, Host, InsertableComponents};

/// Generic game entity script component.
/// Implements the game_entity.wit component definition.
#[derive(Component)]
pub struct EntityScript {
    game_entity: GameEntity,
    store: Store<State>
}

enum EntityScriptCommand {
    RemoveReflectComponent(String),
    InsertComponent(InsertableComponents)
}

struct GameEngineComponent {
    entity: Entity,
    queued_commands: Vec<EntityScriptCommand>
}

impl Host for GameEngineComponent {
    fn remove_component(&mut self, path: String) {
        info!("Removing {path}");
        self.queued_commands.push(EntityScriptCommand::RemoveReflectComponent(path));
    }

    fn insert_components(&mut self, components: Vec<InsertableComponents>) {
        for cmp in components {
            self.queued_commands.push(EntityScriptCommand::InsertComponent(cmp));
        }
    }
}


impl GameEngineComponent {
    fn apply_command_queue(&mut self, commands: &mut Commands) {
        for cmd in self.queued_commands.drain(..) {
            match cmd {
                EntityScriptCommand::RemoveReflectComponent(type_path) => {
                    commands.entity(self.entity).remove_reflect(type_path);
                }
                EntityScriptCommand::InsertComponent(cmp) => {
                    match cmp {
                        InsertableComponents::Interactable(crate::scripting::game_entity::gamejam::game::game_host::Interactable {message, range }) => {
                            commands.entity(self.entity).insert(Interactable {
                                action_hint: message,
                                range
                            });
                        }
                    }
                }
            }
        }
    }
}

struct State {
    host: GameEngineComponent,
}

pub fn create_entity_script(
    script_path: &str,
    engine: &Res<WasmEngine>,
    asset_server: &Res<AssetServer>,
    wasm_scripts: &mut Assets<WasmScriptModuleBytes>,
) -> EntityScript {
    let script: Handle<WasmScriptModuleBytes> =
        asset_server.load(script_path);
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
        State {
            host: GameEngineComponent { entity: Entity::PLACEHOLDER , queued_commands: vec![] },
        },
    );

    let mut linker = Linker::<State>::new(&engine.0);

    add_to_linker(&mut linker, |state: &mut State| {
        &mut state.host
    })
    .unwrap();

    let entity = GameEntity::instantiate(&mut store, &component, &linker).unwrap();

    entity.call_startup(&mut store).unwrap();

    EntityScript {
        game_entity: entity,
        store
    }
}

pub fn wasmwat_system(
    mut commands: Commands,
    mut scripted_entities: Query<(Entity, &mut EntityScript)>,
) {
    for (entity, mut script) in scripted_entities.iter_mut() {
        let EntityScript { game_entity, store } = script.as_mut();
        {
            store.data_mut().host.entity = entity;
            game_entity.call_tick(store.as_context_mut()).unwrap();
        }
        
        store.data_mut().host.apply_command_queue(&mut commands);
    }
}
