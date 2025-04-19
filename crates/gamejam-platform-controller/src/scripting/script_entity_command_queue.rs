use crate::audio::audio_components::{AudioEffect, AudioMusic};
use crate::combat::attackable::Attackable;
use crate::combat::combat_components::ScheduledAttack;
use crate::combat::projectiles::Projectile;
use crate::combat::Enemy;
use crate::game_entities::file_formats::game_entity_definitions::{
    GameEntityDefinitionFile, GameEntityDefinitionFileHandle,
};
use crate::graphics::sprite_collection::SpriteCollection;
use crate::ldtk_entities::player_spawn::RequestedPlayerSpawn;
use crate::movement_systems::movement_components::{EntityInput, FacingDirection, Input};
use crate::player_systems::player_components::{Player, PowerupPogo, PowerupRoll};
use crate::scripting::create_entity_script::create_entity_script;
use crate::scripting::scripted_game_entity::{EntityScript, GameData, ScriptEvent};
use crate::timing::timer_system::add_timer_to_entity;
use crate::timing::timing_component::{TimerComponent, TimerData};
use crate::GameStates;
use avian2d::collision::{Collider, CollisionLayers};
use avian2d::prelude::{LinearVelocity, RigidBody};
use bevy::asset::{AssetServer, Assets};
use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::ecs::reflect::ReflectCommandExt;
use bevy::hierarchy::BuildChildren;
use bevy::log::{error, info};
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Component, Entity, EventWriter, NextState, Query, Res, ResMut, Transform,
    Vec3Swizzles, With,
};
use bevy::time::{Timer, TimerMode};
use bevy_ecs_ldtk::LevelSelection;
use bevy_wasmer_scripting::scripted_entity::WasmEngine;
use bevy_wasmer_scripting::wasm_script_asset::WasmScriptModuleBytes;
use gamejam_bevy_components::Interactable;
use scripted_game_entity::gamejam::game::game_host;
use scripted_game_entity::gamejam::game::game_host::InsertableComponents;
use scripted_game_entity::gamejam::game::game_host::*;
use std::ops::{Add, DerefMut};
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
    SpawnProjectile(Vec2, Vec2, String, Vec<String>),
}

pub fn scripted_entity_command_queue_system(
    mut commands: Commands,
    sprites: Res<SpriteCollection>,
    asset_server: Res<AssetServer>,
    entity_db: Res<Assets<GameEntityDefinitionFile>>,
    entity_db_handle: Res<GameEntityDefinitionFileHandle>,
    wasm_engine: Res<WasmEngine>,
    game_data: Res<GameData>,
    mut wasm_scripts: ResMut<Assets<WasmScriptModuleBytes>>,
    mut level_select: ResMut<LevelSelection>,
    mut event_writer: EventWriter<ScriptEvent>,
    mut input_event_writer: EventWriter<EntityInput>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut query: Query<(
        Entity,
        &mut EntityScript,
        &mut TimerComponent,
        Option<&Transform>,
    )>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.single();
    let entity_db = entity_db
        .get(&entity_db_handle.0)
        .expect("missing entity db file");

    for (entity, mut queue, mut timer, transform) in query.iter_mut() {
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
                next_state.as_mut(),
                timer.deref_mut(),
                entity_db,
                &wasm_engine,
                &game_data,
                &mut wasm_scripts,
                &transform,
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
    next_state: &mut NextState<GameStates>,
    timer_component: &mut TimerComponent,
    entity_db: &GameEntityDefinitionFile,
    wasm_engine: &Res<WasmEngine>,
    game_data: &Res<GameData>,
    wasm_scripts: &mut ResMut<Assets<WasmScriptModuleBytes>>,
    transform: &Option<&Transform>,
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
            entity.try_insert(
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
            next_state.set(GameStates::LoadLevel)
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
        EntityScriptCommand::SpawnProjectile(velocity, offset, prototype, mut script_params) => {
            let Some(transform) = transform else {
                return;
            };

            let prototype = entity_db.entities.get(&prototype).unwrap();
            let mut transform = **transform;
            transform.translation = transform.translation.add(&offset.extend(2.));

            let mut projectile_entity = commands.spawn((
                Projectile::default(),
                LinearVelocity(Vec2::new(velocity.x, velocity.y)),
                transform,
                TimerComponent::default()
            ));

            let mut args = prototype.script_params.clone().unwrap_or_default();
            args.append(&mut script_params);

            let script = create_entity_script(
                projectile_entity.id(),
                prototype.script_path.as_ref().unwrap(),
                &wasm_engine,
                asset_server,
                game_data,
                wasm_scripts.as_mut(),
                Some(args),
                transform.translation.xy(),
            );

            info!("spawned projectile: at {transform:?}");

            projectile_entity.insert(script);
        }
    }
}
