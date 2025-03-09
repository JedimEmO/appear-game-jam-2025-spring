use crate::graphics::animation_system::{SpriteAnimation, SpriteAnimationCompleted};
use crate::graphics::sprite_collection::SpriteCollection;
use crate::ldtk_entities::get_ldtk_enum_field;
use crate::ldtk_entities::interactable::{InteractableInRange, Interacted};
use anyhow::anyhow;
use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use std::time::Duration;
use gamejam_bevy_components::Interactable;
use crate::player_components::{Player, PlayerStats};

pub enum ChestType {
    Small,
    Large,
}

impl TryFrom<String> for ChestType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "chest_small" => Ok(ChestType::Small),
            "chest_large" => Ok(ChestType::Large),
            _ => Err(format!("Invalid chest type: {}", value)),
        }
    }
}

#[derive(Component)]
pub struct Chest {
    pub chest_type: ChestType,
}

#[derive(Component)]
pub struct ChestOpening;

impl TryFrom<&EntityInstance> for Chest {
    type Error = anyhow::Error;

    fn try_from(entity_instance: &EntityInstance) -> Result<Self, Self::Error> {
        Ok(Self {
            chest_type: get_ldtk_enum_field("chest_type", entity_instance)?
                .ok_or(anyhow!("chest_type field not found"))?,
        })
    }
}

pub fn spawn_chest_system(
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    mut query: Query<(Entity, &mut Transform, &EntityInstance), Added<Chest>>,
) {
    for (entity, _transform, _entity_instance) in query.iter_mut() {
        commands
            .entity(entity)
            .insert(
                assets
                    .create_sprite_animation_bundle(
                        "chest_simple",
                        "locked",
                        Duration::from_secs(20),
                        true,
                        false,
                        false,
                    )
                    .unwrap(),
            )
            .insert(Interactable {
                action_hint: "Press <up> to Open".to_string(),
                range: 20.0,
            });
    }
}

pub fn chest_opening_added_observer(
    trigger: Trigger<OnAdd, Interacted>,
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    query: Query<Entity, With<Chest>>,
) {
    let Some(mut entity) = commands.get_entity(trigger.entity()) else {
        return;
    };

    let Ok(_) = query.get(trigger.entity()) else {
        return;
    };

    entity
        .remove::<Sprite>()
        .remove::<SpriteAnimation>()
        .remove::<Interacted>()
        .remove::<Interactable>()
        .remove::<InteractableInRange>()
        .insert(
            assets
                .create_sprite_animation_bundle(
                    "chest_simple",
                    "opening",
                    Duration::from_millis(500),
                    false,
                    false,
                    false,
                )
                .unwrap(),
        );
}

pub fn chest_animation_completed_observer(
    trigger: Trigger<OnAdd, SpriteAnimationCompleted>,
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    query: Query<Entity, With<Chest>>,
    mut player_health: Query<&mut PlayerStats, With<Player>>,
) {
    let Some(mut entity) = commands.get_entity(trigger.entity()) else {
        return;
    };

    if !query.contains(trigger.entity()) {
        return;
    };

    let mut player_stats = player_health.single_mut();
    player_stats.max_health += 2;

    entity
        .remove::<Sprite>()
        .remove::<SpriteAnimation>()
        .remove::<SpriteAnimationCompleted>()
        .insert(
            assets
                .create_sprite_animation_bundle(
                    "chest_simple",
                    "open",
                    Duration::from_secs(1),
                    true,
                    false,
                    false,
                )
                .unwrap(),
        );
}
