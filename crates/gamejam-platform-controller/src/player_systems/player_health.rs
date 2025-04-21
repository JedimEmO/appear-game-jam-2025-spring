use crate::player_systems::player_components::{Player, PlayerStatsMutable, PowerupPogo, PowerupRoll};
use bevy::prelude::{Commands, Entity, NextState, Query, ResMut, With, Without};
use bevy_ecs_ldtk::LevelSelection;
use crate::combat::combat_components::{Health, Stamina};
use crate::GameStates;
use crate::ldtk_entities::player_spawn::RequestedPlayerSpawn;
use crate::player_systems::bonfire::Bonfire;

pub fn player_health_sync_system(
    mut commands: Commands,
    mut level_select: ResMut<LevelSelection>,
    mut next_state: ResMut<NextState<GameStates>>,
    player_stats: Query<&PlayerStatsMutable, Without<Player>>,
    mut player_hp: Query<(Entity, &Stamina, &mut Health, Option<&PowerupPogo>, Option<&PowerupRoll>, &Bonfire), With<Player>>,
) {
    let Ok((player, stamina, mut health, pogo, roll, bonfire)) = player_hp.get_single_mut() else {
        return;
    };

    let Ok(stats) = player_stats.get_single() else {
        return;
    };

    if health.0.current == 0 && health.0.newly_consumed == 0 {
        health.0.current = health.0.max;
        commands.entity(player).insert(RequestedPlayerSpawn { spawn_name: bonfire.spawn_name.clone() });
        *level_select = LevelSelection::index(bonfire.level_index as usize);
        
        next_state.set(GameStates::LoadLevel);
    }

    stats.health.current.set(health.0.current);
    stats.health.max.set(health.0.max);
    stats.health.newly_consumed.set(health.0.newly_consumed);
    stats.stamina.current.set(stamina.0.current);
    stats.stamina.max.set(stamina.0.max);
    stats.stamina.newly_consumed.set(stamina.0.newly_consumed);
    stats.has_pogo.set(pogo.is_some());
    stats.has_rolling.set(roll.is_some());
}
