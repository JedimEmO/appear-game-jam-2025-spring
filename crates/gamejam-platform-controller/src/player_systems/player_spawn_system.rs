use crate::graphics::animation_system::{spawn_animated_sprite_for_entity, SpriteSettings};
use crate::player_components::{Player, PlayerStatsMutable};
use crate::{GameStates, PlayerAssets, PlayerSpawnSettings};
use bevy::prelude::{Camera2d, Commands, NextState, Query, Res, ResMut, Transform, With};
use bevy::utils::default;
use std::time::Duration;
use crate::ldtk_entities::player_spawn::RequestedPlayerSpawn;

pub fn spawn_player_system(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    let mut entity = commands.spawn((
        Player,
        RequestedPlayerSpawn { spawn_name: "game_start".to_string() },
    ));

    spawn_animated_sprite_for_entity(
        &mut entity,
        player_assets.player.clone(),
        player_assets.player_layout.clone(),
        0,
        4,
        Duration::from_millis(1000),
        SpriteSettings {
            repeating: true,
            ..default()
        },
    );

    next_state.set(GameStates::GameLoop);
}

pub fn spawn_player_ui_proxy_system(mut commands: Commands) {
    commands.spawn(PlayerStatsMutable::default());
}
