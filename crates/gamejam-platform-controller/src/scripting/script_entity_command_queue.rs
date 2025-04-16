use crate::audio::audio_components::{AudioEffect, AudioMusic};
use crate::combat::attackable::Attackable;
use crate::combat::combat_components::ScheduledAttack;
use crate::combat::Enemy;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::ldtk_entities::player_spawn::RequestedPlayerSpawn;
use crate::movement_systems::movement_components::{EntityInput, FacingDirection, Input};
use crate::player_systems::player_components::{Player, PowerupPogo, PowerupRoll};
use crate::scripting::scripted_game_entity::{EntityScript, ScriptEvent};
use crate::timing::timer_system::add_timer_to_entity;
use crate::timing::timing_component::{TimerComponent, TimerData};
use avian2d::collision::{Collider, CollisionLayers};
use avian2d::prelude::RigidBody;
use bevy::asset::AssetServer;
use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::ecs::reflect::ReflectCommandExt;
use bevy::hierarchy::BuildChildren;
use bevy::log::{error, info};
use bevy::prelude::{Commands, Component, Entity, EventWriter, Query, Res, ResMut, With};
use bevy::time::{Timer, TimerMode};
use bevy_ecs_ldtk::LevelSelection;
use gamejam_bevy_components::Interactable;
use scripted_game_entity::gamejam::game::game_host;
use scripted_game_entity::gamejam::game::game_host::InsertableComponents;
use scripted_game_entity::gamejam::game::game_host::*;
use std::ops::DerefMut;
use std::time::Duration;

#[derive(Component)]
pub struct TickingEntity(pub Option<f32>);

pub enum EntityScriptCommand {
    RemoveReflectComponent(String),
    InsertComponent(InsertableComponents),
    PlayAnimation {
        sprite_name: String,
        animation_name: String,
        duration: Duration,
        direction: Direction,
        repeat: bool,
    },
    PublishEvent(ScriptEvent),
    ToggleTicking((bool, Option<f32>)),
    DespawnEntity(u64),
    LevelTransition(u32, String),
    RequestTimer(u32, Duration),
    Input(Input),
    Face(FacingDirection),
    ScheduleAttack(ScheduledAttack),
    PlayMusic(String),
    PlaySound(String),
    GrantPlayerPower(String),
}

pub fn scripted_entity_command_queue_system(
    mut commands: Commands,
    sprites: Res<SpriteCollection>,
    asset_server: Res<AssetServer>,
    mut level_select: ResMut<LevelSelection>,
    mut event_writer: EventWriter<ScriptEvent>,
    mut input_event_writer: EventWriter<EntityInput>,
    mut query: Query<(Entity, &mut EntityScript, &mut TimerComponent)>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.single();

    for (entity, mut queue, mut timer) in query.iter_mut() {
        for cmd in queue.store.data_mut().host.queued_commands.drain(..) {
            apply_command(
                player_entity,
                entity,
                cmd,
                asset_server.as_ref(),
                &mut commands,
                &sprites,
                &mut level_select,
                &mut event_writer,
                &mut input_event_writer,
                timer.deref_mut(),
            );
        }
    }
}

fn apply_command(
    player_entity: Entity,
    entity_id: Entity,
    cmd: EntityScriptCommand,
    asset_server: &AssetServer,
    commands: &mut Commands,
    sprites: &Res<SpriteCollection>,
    level_select: &mut ResMut<LevelSelection>,
    event_writer: &mut EventWriter<ScriptEvent>,
    input_event_writer: &mut EventWriter<EntityInput>,
    timer_component: &mut TimerComponent,
) {
    let mut entity = commands.entity(entity_id);

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
            InsertableComponents::RigidBody(type_) => {
                let body_type = match type_ {
                    RigidBodyType::StaticBody => RigidBody::Static,
                    RigidBodyType::Dynamic => RigidBody::Dynamic,
                };

                entity.insert(body_type);
            }
            InsertableComponents::Enemy => {
                entity.insert(Enemy::default());
            }
        },
        EntityScriptCommand::PlayAnimation {
            sprite_name,
            animation_name,
            duration,
            direction,
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
                        match direction {
                            Direction::West => true,
                            _ => false,
                        },
                    )
                    .unwrap(),
            );
        }
        EntityScriptCommand::PublishEvent(evt) => {
            info!("publishing script event: {evt:?}");
            event_writer.send(evt);
        }
        EntityScriptCommand::ToggleTicking((should_tick, distance)) => {
            if should_tick {
                entity.insert(TickingEntity(distance));
            } else {
                entity.remove::<TickingEntity>();
            }
        }
        EntityScriptCommand::DespawnEntity(entity) => {
            commands
                .get_entity(Entity::from_bits(entity))
                .map(|mut e| e.despawn());
        }
        EntityScriptCommand::LevelTransition(level_index, spawn_name) => {
            commands
                .entity(player_entity)
                .insert(RequestedPlayerSpawn { spawn_name });

            **level_select = LevelSelection::index(level_index as usize);
        }
        EntityScriptCommand::RequestTimer(timer, duration) => {
            let data = TimerData {
                timer_name: timer,
                timer: Timer::new(duration, TimerMode::Once),
                on_expiration: None,
            };

            add_timer_to_entity(timer_component, data);
        }
        EntityScriptCommand::Input(input) => {
            input_event_writer.send(EntityInput {
                entity: entity_id,
                input,
            });
        }
        EntityScriptCommand::Face(direction) => {
            entity.insert(direction);
        }
        EntityScriptCommand::ScheduleAttack(mut attack) => {
            attack.attacker = entity_id;
            let mut attack = commands.spawn(attack);
            attack.set_parent(entity_id);
        }
        EntityScriptCommand::PlayMusic(filename) => {
            commands.spawn((
                AudioPlayer::new(asset_server.load(filename)),
                PlaybackSettings::LOOP,
                AudioMusic,
            ));
        }
        EntityScriptCommand::PlaySound(filename) => {
            commands.spawn((
                AudioPlayer::new(asset_server.load(filename)),
                PlaybackSettings::ONCE,
                AudioEffect,
            ));
        }
        EntityScriptCommand::GrantPlayerPower(power) => match power.as_str() {
            "roll" => {
                commands.entity(player_entity).insert(PowerupRoll);
            }
            "pogo" => {
                commands.entity(player_entity).insert(PowerupPogo);
            }
            power => {
                info!("Attempting to grant invalid power {power}")
            }
        },
    }
}
