use crate::player_components::Player;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct PlayerCollidable;

#[derive(Component, Clone, Copy)]
pub struct PlayerCollidableInRangeForCheck;

pub fn player_collidable_system(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCollidable>)>,
    collidable_query: Query<(Entity, &Transform), (With<PlayerCollidable>, Without<Player>)>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    for (entity, entity_transform) in collidable_query.iter() {
        let mut entity_commands = commands.entity(entity);

        if entity_transform.translation.distance(player.translation) > 40. {
            entity_commands.remove::<PlayerCollidableInRangeForCheck>();
            continue;
        }

        entity_commands.insert(PlayerCollidableInRangeForCheck);
    }
}
