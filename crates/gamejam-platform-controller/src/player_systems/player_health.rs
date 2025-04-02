use crate::player_systems::player_components::{Player, PlayerStats, PlayerStatsMutable};
use bevy::prelude::{Query, With, Without};
use crate::combat::combat_components::{Health, Stamina};

pub fn player_health_sync_system(
    player_stats: Query<&PlayerStatsMutable, Without<Player>>,
    player_hp: Query<(&Stamina, &Health), With<Player>>,
) {
    let Ok((stamina, health)) = player_hp.get_single() else {
        return;
    };

    let Ok(stats) = player_stats.get_single() else {
        return;
    };

    if health.0.current == 0 && health.0.newly_consumed == 0 {
        panic!("Game over man. Game over!");
    }

    stats.health.current.set(health.0.current);
    stats.health.max.set(health.0.max);
    stats.health.newly_consumed.set(health.0.newly_consumed);
    stats.stamina.current.set(stamina.0.current);
    stats.stamina.max.set(stamina.0.max);
    stats.stamina.newly_consumed.set(stamina.0.newly_consumed);
}
