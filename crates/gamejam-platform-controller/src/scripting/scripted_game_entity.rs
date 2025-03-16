use crate::ldtk_entities::interactable::{InteractableInRange, Interacted};
use crate::scripting::script_entity_command_queue::{EntityScriptCommand, TickingEntity};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::prelude::{
    Bundle, Commands, Component, Entity, Event, EventReader, OnAdd, Query, Res, Resource, Trigger,
    With,
};
use bevy_wasmer_scripting::scripted_entity::WasmEngine;
use bevy_wasmer_scripting::wasm_script_asset::WasmScriptModuleBytes;
use scripted_game_entity::exports::gamejam::game::entity_resource::StartupSettings;
use scripted_game_entity::gamejam::game::game_host;
use scripted_game_entity::gamejam::game::game_host::{add_to_linker, Host, InsertableComponents};
use scripted_game_entity::GameEntityWorld;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasmtime::component::{Linker, ResourceAny};
use wasmtime::{AsContextMut, Store};

#[derive(Default)]
pub struct GameState {
    pub strings: BTreeMap<String, String>,
    pub ints: BTreeMap<String, i32>,
}

#[derive(Resource, Default)]
pub struct GameData {
    pub game_state: Arc<Mutex<GameState>>,
}

/// Generic game entity script component.
/// Implements the game_entity.wit component definition.
#[derive(Component)]
pub struct EntityScript {
    pub game_entity: GameEntityWorld,
    pub entity_resource: ResourceAny,
    pub store: Store<State>,
}

impl EntityScript {
    pub fn animation_finished(&mut self, animation_name: &str) {
        let guest = self.game_entity.gamejam_game_entity_resource();
        let entity_resource_guest = guest.game_entity();

        entity_resource_guest
            .call_animation_finished(
                self.store.as_context_mut(),
                self.entity_resource,
                animation_name,
            )
            .unwrap();
    }

    pub fn interact(&mut self) {
        let guest = self.game_entity.gamejam_game_entity_resource();
        let entity_resource_guest = guest.game_entity();

        entity_resource_guest
            .call_interacted(self.store.as_context_mut(), self.entity_resource)
            .unwrap();
    }

    pub fn attacked(&mut self) {
        let guest = self.game_entity.gamejam_game_entity_resource();
        let entity_resource_guest = guest.game_entity();

        entity_resource_guest
            .call_attacked(self.store.as_context_mut(), self.entity_resource)
            .unwrap();
    }
}

pub struct GameEngineComponent {
    pub entity: Entity,
    pub queued_commands: Vec<EntityScriptCommand>,
    pub game_state: Arc<Mutex<GameState>>,
}

unsafe impl Send for GameEngineComponent {}
unsafe impl Sync for GameEngineComponent {}

impl Host for GameEngineComponent {
    fn remove_component(&mut self, path: String) {
        self.queued_commands
            .push(EntityScriptCommand::RemoveReflectComponent(path));
    }

    fn insert_components(&mut self, components: Vec<InsertableComponents>) {
        for cmp in components {
            self.queued_commands
                .push(EntityScriptCommand::InsertComponent(cmp));
        }
    }

    fn play_animation(
        &mut self,
        sprite_name: String,
        animation_name: String,
        duration_millis: u32,
        flip_x: bool,
        repeat: bool,
    ) {
        self.queued_commands
            .push(EntityScriptCommand::PlayAnimation {
                sprite_name,
                animation_name,
                duration: Duration::from_millis(duration_millis as u64),
                flip_x,
                repeat,
            });
    }
    fn publish_event(&mut self, evt: game_host::Event) {
        self.queued_commands
            .push(EntityScriptCommand::PublishEvent(ScriptEvent {
                topic: evt.topic,
                data: match evt.data {
                    game_host::EventData::Trigger(topic) => ScriptEventData::Trigger(topic),
                },
            }));
    }

    fn set_ticking(&mut self, ticking: bool) {
        self.queued_commands
            .push(EntityScriptCommand::ToggleTicking(ticking));
    }

    fn despawn_entity(&mut self, entity_id: u64) {
        self.queued_commands
            .push(EntityScriptCommand::DespawnEntity(entity_id));
    }

    fn get_game_data_kv(&mut self, key: String) -> Option<String> {
        self.game_state.lock().unwrap().strings.get(&key).cloned()
    }

    fn set_game_data_kv(&mut self, key: String, value: String) -> Option<String> {
        self.game_state.lock().unwrap().strings.insert(key, value)
    }

    fn get_game_data_kv_int(&mut self, key: String) -> Option<i32> {
        self.game_state.lock().unwrap().ints.get(&key).cloned()
    }

    fn set_game_data_kv_int(&mut self, key: String, value: i32) -> Option<i32> {
        self.game_state.lock().unwrap().ints.insert(key, value)
    }
}

impl GameEngineComponent {}

pub struct State {
    pub host: GameEngineComponent,
}

pub fn create_entity_script(
    entity: Entity,
    script_path: &str,
    engine: &Res<WasmEngine>,
    asset_server: &Res<AssetServer>,
    game_data: &Res<GameData>,
    wasm_scripts: &mut Assets<WasmScriptModuleBytes>,
    script_params: Option<Vec<String>>,
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
        State {
            host: GameEngineComponent {
                entity: Entity::PLACEHOLDER,
                queued_commands: vec![],
                game_state: game_data.game_state.clone(),
            },
        },
    );

    let mut linker = Linker::<State>::new(&engine.0);

    add_to_linker(&mut linker, |state: &mut State| &mut state.host).unwrap();

    let settings = StartupSettings {
        params: script_params,
        self_entity_id: entity.to_bits(),
    };

    let entity = GameEntityWorld::instantiate(&mut store, &component, &linker).unwrap();
    let guest = entity.gamejam_game_entity_resource();
    let entity_resource_guest = guest.game_entity();

    let entity_resource = entity_resource_guest
        .call_constructor(&mut store, &settings)
        .unwrap();

    EntityScript {
        game_entity: entity,
        entity_resource,
        store,
    }
}

pub fn tick_scripted_entity_system(
    mut scripted_entities: Query<(Entity, &mut EntityScript), With<TickingEntity>>,
) {
    for (_entity, mut script) in scripted_entities.iter_mut() {
        let EntityScript {
            game_entity,
            store,
            entity_resource,
        } = script.as_mut();
        {
            let guest = game_entity.gamejam_game_entity_resource();
            let entity_resource_guest = guest.game_entity();

            entity_resource_guest
                .call_tick(store.as_context_mut(), *entity_resource)
                .unwrap();
        }
    }
}

pub fn script_interaction_observer(
    trigger: Trigger<OnAdd, Interacted>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut EntityScript), With<InteractableInRange>>,
) {
    for (entity, mut script) in query.iter_mut() {
        if entity != trigger.entity() {
            continue;
        }

        script.interact();
        commands.entity(entity).remove::<Interacted>();
    }
}

#[derive(Debug)]
pub enum ScriptEventData {
    Trigger(u32),
}

#[derive(Event, Debug)]
pub struct ScriptEvent {
    pub topic: u32,
    pub data: ScriptEventData,
}

pub fn game_entity_script_event_system(
    mut evt: EventReader<ScriptEvent>,
    mut script_entities_query: Query<&mut EntityScript>,
) {
    for evt in evt.read() {
        for mut entity in script_entities_query.iter_mut() {
            let EntityScript {
                game_entity,
                store,
                entity_resource,
            } = entity.as_mut();
            let guest = game_entity.gamejam_game_entity_resource();
            let entity_resource_guest = guest.game_entity();

            entity_resource_guest
                .call_receive_event(
                    store.as_context_mut(),
                    *entity_resource,
                    game_host::Event {
                        topic: evt.topic,
                        data: match evt.data {
                            ScriptEventData::Trigger(event_id) => {
                                game_host::EventData::Trigger(event_id)
                            }
                        },
                    },
                )
                .unwrap();
        }
    }
}
