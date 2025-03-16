use crate::enemies::attackable::Attackable;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::scripting::game_entity::gamejam::game::game_host;
use crate::scripting::game_entity::gamejam::game::game_host::InsertableComponents;
use crate::scripting::scripted_game_entity::{EntityScript, ScriptEvent};
use avian2d::collision::{Collider, CollisionLayers};
use avian2d::prelude::RigidBody;
use bevy::ecs::reflect::ReflectCommandExt;
use bevy::log::info;
use bevy::prelude::{Commands, Component, Entity, EventWriter, Query, Res};
use gamejam_bevy_components::Interactable;
use std::time::Duration;

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
}

pub fn scripted_entity_command_queue_system(
    mut commands: Commands,
    sprites: Res<SpriteCollection>,
    mut event_writer: EventWriter<ScriptEvent>,
    mut query: Query<(Entity, &mut EntityScript)>,
) {
    for (entity, mut queue) in query.iter_mut() {
        for cmd in queue.store.data_mut().host.queued_commands.drain(..) {
            apply_command(entity, cmd, &mut commands, &sprites, &mut event_writer);
        }
    }
}

fn apply_command(
    entity: Entity,
    cmd: EntityScriptCommand,
    commands: &mut Commands,
    sprites: &Res<SpriteCollection>,
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
    }
}
