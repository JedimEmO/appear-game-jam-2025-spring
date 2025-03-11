use crate::graphics::sprite_collection::SpriteCollection;
use crate::ldtk_entities::interactable::{InteractableInRange, Interacted};
use crate::scripting::game_entity::gamejam::game::game_host;
use crate::scripting::game_entity::gamejam::game::game_host::{
    add_to_linker, Host, InsertableComponents,
};
use crate::scripting::game_entity::GameEntity;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::reflect::ReflectCommandExt;
use bevy::log::info;
use bevy::prelude::{Bundle, Commands, Component, Entity, OnAdd, Query, Res, Trigger, With};
use bevy_wasmer_scripting::scripted_entity::WasmEngine;
use bevy_wasmer_scripting::wasm_script_asset::WasmScriptModuleBytes;
use gamejam_bevy_components::Interactable;
use std::time::{Duration, Instant};
use avian2d::prelude::Collider;
use wasmtime::component::Linker;
use wasmtime::{AsContextMut, Store};
use crate::enemies::attackable::Attackable;

/// Generic game entity script component.
/// Implements the game_entity.wit component definition.
#[derive(Component)]
pub struct EntityScript {
    pub game_entity: GameEntity,
    pub store: Store<State>,
}

impl EntityScript {
    pub fn drain_command_queue(&mut self, cmds: &mut Commands, sprites: &Res<SpriteCollection>) {
        self.store
            .data_mut()
            .host
            .apply_command_queue(cmds, sprites);
    }

    pub fn animation_finished(
        &mut self,
        commands: &mut Commands,
        animation_name: &str,
        sprites: &Res<SpriteCollection>,
    ) {
        self.game_entity
            .call_animation_finished(self.store.as_context_mut(), animation_name)
            .unwrap();
        self.drain_command_queue(commands, sprites);
    }

    pub fn interact(&mut self, commands: &mut Commands, sprites: &Res<SpriteCollection>) {
        self.game_entity
            .call_interacted(self.store.as_context_mut())
            .unwrap();
        self.drain_command_queue(commands, sprites);
    }

    pub fn attacked(&mut self, commands: &mut Commands, sprites: &Res<SpriteCollection>) {
        self.game_entity
            .call_attacked(self.store.as_context_mut())
            .unwrap();
        self.drain_command_queue(commands, sprites);
    }
}

enum EntityScriptCommand {
    RemoveReflectComponent(String),
    InsertComponent(InsertableComponents),
    PlayAnimation {
        sprite_name: String,
        animation_name: String,
        duration: Duration,
        flip_x: bool,
        repeat: bool
    },
}

struct GameEngineComponent {
    entity: Entity,
    queued_commands: Vec<EntityScriptCommand>,
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
        repeat: bool
    ) {
        self.queued_commands
            .push(EntityScriptCommand::PlayAnimation {
                sprite_name,
                animation_name,
                duration: Duration::from_millis(duration_millis as u64),
                flip_x,
                repeat
            });
    }
}

impl GameEngineComponent {
    fn apply_command_queue(&mut self, commands: &mut Commands, sprites: &Res<SpriteCollection>) {
        for cmd in self.queued_commands.drain(..) {
            match cmd {
                EntityScriptCommand::RemoveReflectComponent(type_path) => {
                    commands.entity(self.entity).remove_reflect(type_path);
                }
                EntityScriptCommand::InsertComponent(cmp) => match cmp {
                    InsertableComponents::Interactable(game_host::Interactable {
                        message,
                        range,
                    }) => {
                        commands.entity(self.entity).insert(Interactable {
                            action_hint: message,
                            range,
                        });
                    },
                    InsertableComponents::Attackable => {
                        commands.entity(self.entity).insert(Attackable);
                    },
                    InsertableComponents::Collider(c) => {
                        commands.entity(self.entity).insert(Collider::rectangle(c.width, c.height));
                    }
                },
                EntityScriptCommand::PlayAnimation {
                    sprite_name,
                    animation_name,
                    duration,
                    flip_x,
                    repeat
                } => {
                    commands.entity(self.entity).insert(
                        sprites
                            .create_sprite_animation_bundle(
                                &sprite_name,
                                &animation_name,
                                duration,
                                repeat,
                                false,
                                flip_x,
                            )
                            .unwrap(),
                    );
                }
            }
        }
    }
}

pub struct State {
    host: GameEngineComponent,
}

pub fn create_entity_script(
    script_path: &str,
    engine: &Res<WasmEngine>,
    asset_server: &Res<AssetServer>,
    wasm_scripts: &mut Assets<WasmScriptModuleBytes>,
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

    entity.call_startup(&mut store).unwrap();

    EntityScript {
        game_entity: entity,
        store,
    }
}

pub fn wasmwat_system(
    sprites: Res<SpriteCollection>,
    mut commands: Commands,
    mut scripted_entities: Query<(Entity, &mut EntityScript)>,
) {
    for (entity, mut script) in scripted_entities.iter_mut() {
        let EntityScript { game_entity, store } = script.as_mut();
        {
            store.data_mut().host.entity = entity;
            game_entity.call_tick(store.as_context_mut()).unwrap();
        }

        store
            .data_mut()
            .host
            .apply_command_queue(&mut commands, &sprites);
    }
}

pub fn script_interaction_observer(
    trigger: Trigger<OnAdd, Interacted>,
    mut commands: Commands,
    sprites: Res<SpriteCollection>,
    mut query: Query<(Entity, &mut EntityScript), With<InteractableInRange>>,
) {
    for (entity, mut script) in query.iter_mut() {
        if entity != trigger.entity() {
            continue;
        }

        script.interact(&mut commands, &sprites);
        commands.entity(entity).remove::<Interacted>();
    }
}
