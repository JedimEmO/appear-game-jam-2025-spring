use crate::combat::attackable::Attackable;
use crate::combat::{Dying, HitPoints};
use crate::player_systems::player_components::Player;
use crate::scripting::scripted_game_entity::EntityScript;
use avian2d::collision::CollisionLayers;
use bevy::prelude::*;

pub fn hit_points_system(
    mut commands: Commands,
    mut entities: Query<(Entity, &HitPoints, Option<&mut EntityScript>), Without<Player>>,
) {
    for (entity, hp, script) in entities.iter_mut() {
        if hp.hp == 0 {
            commands
                .entity(entity)
                .insert(Dying)
                .insert(CollisionLayers::new(0b01000, 0b00100))
                .remove::<HitPoints>()
                .remove::<Attackable>();

            if let Some(mut script) = script {
                script.killed();
            }
        }
    }
}
