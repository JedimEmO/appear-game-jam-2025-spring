use crate::enemies::attackable::Attackable;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::ldtk_entities::interactable::{InteractableInRange, Interacted};
use crate::scripting::game_entity::gamejam::game::game_host;
use crate::scripting::game_entity::gamejam::game::game_host::{
    add_to_linker, Host, InsertableComponents,
};
use crate::scripting::game_entity::GameEntity;
use crate::scripting::script_entity_command_queue::EntityScriptCommand;
use avian2d::collision::CollisionLayers;
use avian2d::prelude::{Collider, RigidBody};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::reflect::ReflectCommandExt;
use bevy::log::info;
use bevy::prelude::{
    Bundle, Commands, Component, Entity, Event, EventReader, EventWriter, OnAdd, Query, Res,
    Trigger, With,
};
use bevy_wasmer_scripting::scripted_entity::WasmEngine;
use bevy_wasmer_scripting::wasm_script_asset::WasmScriptModuleBytes;
use gamejam_bevy_components::Interactable;
use std::time::Duration;
use wasmtime::component::Linker;
use wasmtime::{AsContextMut, Store};

/// Generic game entity script component.
/// Implements the game_entity.wit component definition.
#[derive(Component)]
pub struct EntityScript {
    pub game_entity: GameEntity,
    pub store: Store<State>,
}

impl EntityScript {
    pub fn animation_finished(&mut self, animation_name: &str) {
        self.game_entity
            .call_animation_finished(self.store.as_context_mut(), animation_name)
            .unwrap();
    }

    pub fn interact(&mut self) {
        self.game_entity
            .call_interacted(self.store.as_context_mut())
            .unwrap();
    }

    pub fn attacked(&mut self) {
        self.game_entity
            .call_attacked(self.store.as_context_mut())
            .unwrap();
    }
}

pub struct GameEngineComponent {
    pub entity: Entity,
    pub queued_commands: Vec<EntityScriptCommand>,
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
}

impl GameEngineComponent {}

pub struct State {
    pub host: GameEngineComponent,
}

pub fn create_entity_script(
    script_path: &str,
    engine: &Res<WasmEngine>,
    asset_server: &Res<AssetServer>,
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
            },
        },
    );

    let mut linker = Linker::<State>::new(&engine.0);

    add_to_linker(&mut linker, |state: &mut State| &mut state.host).unwrap();

    let entity = GameEntity::instantiate(&mut store, &component, &linker).unwrap();

    entity
        .call_startup(&mut store, script_params.as_deref())
        .unwrap();

    EntityScript {
        game_entity: entity,
        store,
    }
}

pub fn wasmwat_system(mut scripted_entities: Query<(Entity, &mut EntityScript)>) {
    for (entity, mut script) in scripted_entities.iter_mut() {
        let EntityScript { game_entity, store } = script.as_mut();
        {
            store.data_mut().host.entity = entity;
            game_entity.call_tick(store.as_context_mut()).unwrap();
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
            let EntityScript { game_entity, store } = entity.as_mut();

            game_entity
                .call_receive_event(
                    store.as_context_mut(),
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
