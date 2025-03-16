use crate::enemies::attackable::Attackable;
use crate::enemies::{Dying, HitPoints};
use crate::player_components::Player;
use avian2d::collision::CollisionLayers;
use bevy::prelude::*;

pub fn hit_points_system(
    mut commands: Commands,
    entities: Query<(Entity, &HitPoints), Without<Player>>,
) {
    for (entity, hp) in entities.iter() {
        if hp.hp == 0 {
            commands
                .entity(entity)
                .insert(Dying)
                .insert(CollisionLayers::new(0b01000, 0b00100))
                .remove::<HitPoints>()
                .remove::<Attackable>();
        }
    }
}
