use crate::combat::combat_components::ScheduledAttack;
use crate::ldtk_entities::interactable::{InteractableInRange, Interacted};
use crate::movement_systems::movement_components::{FacingDirection, Input};
use crate::player_systems::player_components::Player;
use crate::scripting::script_entity_command_queue::{EntityScriptCommand, TickingEntity};
use crate::scripting::scripted_game_entity::game_host::Vector;
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Component, Entity, Event, EventReader, OnAdd, Query, Res, Resource, Time, Transform,
    Trigger, With,
};
use bevy::time::TimerMode;
use scripted_game_entity::exports::gamejam::game::entity_resource::EntityEvent;
use scripted_game_entity::gamejam::game::game_host::Direction;
use scripted_game_entity::gamejam::game::game_host::{self, EntityUniform};
use scripted_game_entity::gamejam::game::game_host::{Host, InsertableComponents};
use scripted_game_entity::GameEntityWorld;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasmtime::component::ResourceAny;
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
    pub store: Store<GameEntityState>,
}

pub struct GameEntityHost {
    pub entity: Entity,
    pub queued_commands: Vec<EntityScriptCommand>,
    pub game_state: Arc<Mutex<GameState>>,
    pub player_uniform: EntityUniform,
    pub self_uniform: EntityUniform,
}

pub struct GameEntityState {
    pub host: GameEntityHost,
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

    pub fn timer_callback(&mut self, timer: u32) {
        let guest = self.game_entity.gamejam_game_entity_resource();
        let entity_resource_guest = guest.game_entity();

        entity_resource_guest
            .call_timer_callback(self.store.as_context_mut(), self.entity_resource, timer)
            .unwrap();
    }

    pub fn killed(&mut self) {
        self.dispatch_entity_event(EntityEvent::Killed);
    }

    pub fn dispatch_entity_event(&mut self, event: EntityEvent) {
        let guest = self.game_entity.gamejam_game_entity_resource();
        let entity_resource_guest = guest.game_entity();

        entity_resource_guest
            .call_receive_entity_event(self.store.as_context_mut(), self.entity_resource, event)
            .unwrap();
    }
}

unsafe impl Send for GameEntityHost {}
unsafe impl Sync for GameEntityHost {}

impl Host for GameEntityHost {
    fn publish_event(&mut self, evt: game_host::Event) {
        self.queued_commands
            .push(EntityScriptCommand::PublishEvent(ScriptEvent {
                topic: evt.topic,
                data: match evt.data {
                    game_host::EventData::Trigger(topic) => ScriptEventData::Trigger(topic),
                },
            }));
    }

    fn set_ticking(&mut self, ticking: bool, distance: Option<f32>) {
        self.queued_commands
            .push(EntityScriptCommand::ToggleTicking((ticking, distance)));
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
        direction: Direction,
        repeat: bool,
    ) {
        self.queued_commands
            .push(EntityScriptCommand::PlayAnimation {
                sprite_name,
                animation_name,
                duration: Duration::from_millis(duration_millis as u64),
                direction,
                repeat,
            });
    }

    fn level_transition(&mut self, idx: u32, target: String) {
        self.queued_commands
            .push(EntityScriptCommand::LevelTransition(idx, target));
    }

    fn request_timer_callback(&mut self, timer: u32, millis: u32) {
        self.queued_commands.push(EntityScriptCommand::RequestTimer(
            timer,
            Duration::from_millis(millis as u64),
        ))
    }

    fn despawn_entity(&mut self, entity_id: u64) {
        self.queued_commands
            .push(EntityScriptCommand::DespawnEntity(entity_id));
    }

    fn face_direction(
        &mut self,
        direction: scripted_game_entity::gamejam::game::game_host::Direction,
    ) {
        self.queued_commands
            .push(EntityScriptCommand::Face(match direction {
                Direction::West => FacingDirection::West,
                _ => FacingDirection::East,
            }));
    }

    fn get_player_uniform(&mut self) -> EntityUniform {
        self.player_uniform
    }

    fn get_self_uniform(&mut self) -> EntityUniform {
        self.self_uniform
    }

    fn can_see_player(&mut self) -> bool {
        todo!()
    }

    fn send_input(&mut self, input: game_host::Input) {
        let input = match input {
            game_host::Input::Movement(dir) => Input::Move(Vec2::new(dir.0, dir.1)),
            game_host::Input::Jump => Input::Jump,
        };

        self.queued_commands.push(EntityScriptCommand::Input(input))
    }

    fn schedule_attack(
        &mut self,
        delay: u32,
        damage: u32,
        force: f32,
        point: (f32, f32),
        vector: (f32, f32),
    ) {
        self.queued_commands
            .push(EntityScriptCommand::ScheduleAttack(ScheduledAttack {
                attacker: Entity::PLACEHOLDER,
                delay: bevy::prelude::Timer::from_seconds(
                    Duration::from_millis(delay as u64).as_secs_f32(),
                    TimerMode::Once,
                ),
                damage,
                force,
                origin: Vec2::new(point.0, point.1),
                vector: Vec2::new(vector.0, vector.1),
            }))
    }
    fn play_music(&mut self, filename: String) {
        self.queued_commands
            .push(EntityScriptCommand::PlayMusic(filename));
    }
    fn play_sound_once(&mut self, filename: String) {
        self.queued_commands
            .push(EntityScriptCommand::PlaySound(filename));
    }

    fn grant_player_power(&mut self, power: String) {
        self.queued_commands
            .push(EntityScriptCommand::GrantPlayerPower(power));
    }
    fn spawn_projectile(
        &mut self,
        velocity: Vector,
        offset: Vector,
        projectile_prototype: String,
        script_params: Vec<String>,
    ) {
        self.queued_commands
            .push(EntityScriptCommand::SpawnProjectile(
                Vec2::new(velocity.x, velocity.y),
                Vec2::new(offset.x, offset.y),
                projectile_prototype,
                script_params,
            ))
    }
}

pub fn scripted_entity_uniform_system(
    player: Query<(&Transform, &FacingDirection), With<Player>>,
    mut entities: Query<(&mut EntityScript, &Transform)>,
) {
    let (player_transform, player_direction) = player.single();

    for (mut script, transform) in entities.iter_mut() {
        let data = script.store.data_mut();

        data.host.player_uniform.position = (
            player_transform.translation.x,
            player_transform.translation.y,
        );
        data.host.player_uniform.facing = match player_direction {
            FacingDirection::West => Direction::West,
            _ => Direction::East,
        };

        data.host.self_uniform.position = (transform.translation.x, transform.translation.y);
    }
}

pub fn tick_scripted_entity_system(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    mut scripted_entities: Query<(
        Entity,
        &mut EntityScript,
        &TickingEntity,
        Option<&Transform>,
    )>,
) {
    let player = player.single();
    let delta_t = time.elapsed_secs();

    for (_entity, mut script, TickingEntity(distance), transform) in scripted_entities.iter_mut() {
        let EntityScript {
            game_entity,
            store,
            entity_resource,
        } = script.as_mut();
        {
            let in_range = if let Some(distance) = distance {
                if transform.is_none()
                    || transform.unwrap().translation.distance(player.translation) > *distance
                {
                    false
                } else {
                    true
                }
            } else {
                true
            };

            if !in_range {
                continue;
            }

            let guest = game_entity.gamejam_game_entity_resource();
            let entity_resource_guest = guest.game_entity();

            entity_resource_guest
                .call_tick(store.as_context_mut(), *entity_resource, delta_t)
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
