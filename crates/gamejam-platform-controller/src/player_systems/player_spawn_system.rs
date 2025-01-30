use crate::graphics::animation_system::{spawn_animated_sprite_for_entity, SpriteSettings};
use crate::player_components::Player;
use crate::{GameStates, PlayerAssets, PlayerSpawnEntity, PlayerSpawnSettings};
use bevy::prelude::{Added, Commands, NextState, Query, Res, ResMut, Transform};
use std::time::Duration;
use bevy::utils::default;

pub fn spawn_player_system(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    player_spawn_settings: Res<PlayerSpawnSettings>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    let mut entity = commands.spawn((
        Player,
        Transform::from_xyz(
            player_spawn_settings.position.x,
            player_spawn_settings.position.y,
            1.,
        ),
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
        }
    );

    next_state.set(GameStates::GameLoop);
}

pub fn update_player_spawn(
    mut player_spawn_info: ResMut<PlayerSpawnSettings>,
    query: Query<&Transform, Added<PlayerSpawnEntity>>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    player_spawn_info.position = transform.translation.truncate();
}
