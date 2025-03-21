use crate::enemies::attackable::Attackable;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::scripting::scripted_game_entity::{EntityScript, ScriptEvent};
use avian2d::collision::{Collider, CollisionLayers};
use avian2d::prelude::RigidBody;
use bevy::ecs::component::Tick;
use bevy::ecs::reflect::ReflectCommandExt;
use bevy::log::info;
use bevy::prelude::{Commands, Component, Entity, EventWriter, Query, Res, ResMut, With};
use gamejam_bevy_components::Interactable;
use scripted_game_entity::gamejam::game::game_host;
use scripted_game_entity::gamejam::game::game_host::InsertableComponents;
use scripted_game_entity::*;
use std::time::Duration;
use bevy::time::{Timer, TimerMode};
use bevy_ecs_ldtk::LevelSelection;
use crate::ldtk_entities::player_spawn::RequestedPlayerSpawn;
use crate::player_components::Player;
use crate::timing::timing_component::TimerComponent;

#[derive(Component)]
pub struct TickingEntity;

pub enum EntityScriptCommand {
    RemoveReflectComponent(String),
    InsertComponent(InsertableComponents),
    PlayAnimation {
        sprite_name: String,
        animation_name: String,
        duration: Duration,
        flip_x: bool,
        repeat: bool,
    },
    PublishEvent(ScriptEvent),
    ToggleTicking(bool),
    DespawnEntity(u64),
    LevelTransition(u32, String),
    RequestTimer(u32, Duration),
}

pub fn scripted_entity_command_queue_system(
    mut commands: Commands,
    sprites: Res<SpriteCollection>,
    mut level_select: ResMut<LevelSelection>,
    mut event_writer: EventWriter<ScriptEvent>,
    mut query: Query<(Entity, &mut EntityScript)>,
    player: Query<Entity, With<Player>>
) {
    let player_entity = player.single();

    for (entity, mut queue) in query.iter_mut() {
        for cmd in queue.store.data_mut().host.queued_commands.drain(..) {
            apply_command(player_entity, entity, cmd, &mut commands, &sprites, &mut level_select, &mut event_writer);
        }
    }
}

fn apply_command(
    player_entity: Entity,
    entity: Entity,
    cmd: EntityScriptCommand,
    commands: &mut Commands,
    sprites: &Res<SpriteCollection>,
    level_select: &mut ResMut<LevelSelection>,
    event_writer: &mut EventWriter<ScriptEvent>,
) {
    let mut entity = commands.entity(entity);

    match cmd {
        EntityScriptCommand::RemoveReflectComponent(type_path) => {
            entity.remove_reflect(type_path);
        }
        EntityScriptCommand::InsertComponent(cmp) => match cmp {
            InsertableComponents::Interactable(game_host::Interactable { message, range }) => {
                entity.insert(Interactable {
                    action_hint: message,
                    range,
                });
            }
            InsertableComponents::Attackable => {
                entity.insert(Attackable);
            }
            InsertableComponents::Collider(c) => {
                entity.insert(Collider::rectangle(c.width, c.height));

                if c.physical {
                    entity.insert((CollisionLayers::new(0b00100, 0b01101), RigidBody::Static));
                }
            }
        },
        EntityScriptCommand::PlayAnimation {
            sprite_name,
            animation_name,
            duration,
            flip_x,
            repeat,
        } => {
            entity.insert(
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
        EntityScriptCommand::PublishEvent(evt) => {
            info!("publishing script event: {evt:?}");
            event_writer.send(evt);
        }
        EntityScriptCommand::ToggleTicking(should_tick) => {
            if should_tick {
                entity.insert(TickingEntity);
            } else {
                entity.remove::<TickingEntity>();
            }
        }
        EntityScriptCommand::DespawnEntity(entity) => {
            commands
                .get_entity(Entity::from_bits(entity))
                .map(|mut e| e.despawn());
        },
        EntityScriptCommand::LevelTransition(level_index, spawn_name) => {
            commands.entity(player_entity).insert(RequestedPlayerSpawn {
                spawn_name,
            });

            **level_select = LevelSelection::index(level_index as usize);
        }
        EntityScriptCommand::RequestTimer(timer, duration) => {
            entity.insert(TimerComponent { timer_name: timer, timer: Timer::new(duration, TimerMode::Once)});
        }
    }
}
