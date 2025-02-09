use bevy::prelude::{Query, With, Without};
use crate::enemies::HitPoints;
use crate::player_components::{Player, PlayerStats, PlayerStatsMutable};

pub fn player_health_sync_system(
    player_stats: Query<(&PlayerStatsMutable), Without<Player>>,
    player_hp: Query<(&HitPoints, &PlayerStats), With<Player>>
) {
    let Ok(hp) = player_hp.get_single() else {
        return;
    };

    let Ok(stats) = player_stats.get_single() else {
        return;
    };

    stats.hp.set(hp.0.hp);
    stats.max_hp.set(hp.1.max_health);
}