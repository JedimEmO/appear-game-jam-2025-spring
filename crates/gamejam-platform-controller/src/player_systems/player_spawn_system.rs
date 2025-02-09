use crate::graphics::animation_system::{spawn_animated_sprite_for_entity, SpriteSettings};
use crate::player_components::{Player, PlayerStatsMutable};
use crate::{GameStates, PlayerAssets, PlayerSpawnEntity, PlayerSpawnSettings};
use bevy::prelude::{Added, Camera2d, Commands, NextState, Query, Res, ResMut, Transform, With};
use bevy::utils::default;
use std::time::Duration;

pub fn spawn_player_system(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    player_spawn_settings: Res<PlayerSpawnSettings>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    let mut camera = camera.single_mut();
    camera.translation.x = player_spawn_settings.position.x;
    camera.translation.y = player_spawn_settings.position.y;

    let mut entity = commands.spawn((
        Player,
        Transform::from_xyz(
            player_spawn_settings.position.x,
            player_spawn_settings.position.y,
            2.,
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
        },
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

pub fn spawn_player_ui_proxy_system(mut commands: Commands) {
    commands.spawn(PlayerStatsMutable::default());
}
