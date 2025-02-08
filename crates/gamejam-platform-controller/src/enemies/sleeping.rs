use bevy::prelude::*;
use crate::enemies::{Enemy, Sleeping};
use crate::player_components::Player;

pub fn sleeping_enemy_system(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform, Option<&Sleeping>), (With<Enemy>, Without<Player>)>,
    player: Query<&Transform, With<Player>>
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };

    for (entity, enemy_transform, sleeping) in enemies.iter() {
        let distance = player_transform.translation.distance(enemy_transform.translation);

        if distance > 200. {
            commands.entity(entity).insert(Sleeping);
        } else {
            commands.entity(entity).remove::<Sleeping>();
        }
    }
}