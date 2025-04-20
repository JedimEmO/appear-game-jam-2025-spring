use crate::combat::attackable::Attackable;
use crate::combat::{Dying};
use crate::player_systems::player_components::Player;
use crate::scripting::scripted_game_entity::EntityScript;
use avian2d::collision::CollisionLayers;
use bevy::prelude::*;
use crate::combat::combat_components::{Boss, BossHealth, Health};

pub fn hit_points_system(
    mut commands: Commands,
    mut entities: Query<(Entity, &Health, Option<&mut EntityScript>), Without<Player>>,
) {
    for (entity, hp, script) in entities.iter_mut() {
        if hp.0.current == 0 {
            commands
                .entity(entity)
                .insert(Dying)
                .insert(CollisionLayers::new(0b01000, 0b00100))
                .remove::<Health>()
                .remove::<Attackable>();

            if let Some(mut script) = script {
                script.killed();
            }
        }
    }
}

pub fn boss_health_system(
    boss_health: ResMut<BossHealth>,
    health: Query<&mut Health, (With<Boss>, Without<Player>)>,
) {
    for hp in health.iter() {
        boss_health.1.set(true);
        boss_health.0.current.set(hp.0.current);
        boss_health.0.newly_consumed.set(hp.0.newly_consumed);
        boss_health.0.max.set(hp.0.max);
    }
}