use crate::combat::attackable::Attackable;
use crate::combat::{Dying};
use crate::player_systems::player_components::Player;
use crate::scripting::scripted_game_entity::EntityScript;
use avian2d::collision::CollisionLayers;
use bevy::prelude::*;
use crate::combat::combat_components::Health;

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
