use bevy::prelude::*;
use crate::player_components::Player;

#[derive(Default, Component)]
pub struct PlayerSpawnEntity {
    pub spawn_name: String
}

#[derive(Default, Component)]
pub struct RequestedPlayerSpawn {
    pub spawn_name: String
}


pub fn move_player_to_spawn(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &RequestedPlayerSpawn), With<Player>>,
    mut camera_query: Query<(&mut Transform), (With<Camera2d>, Without<Player>)>,
    spawn_query: Query<(&Transform, &PlayerSpawnEntity), (Without<Player>, Without<Camera2d>)>
) {
    let Ok((player_entity, mut player_transform, request)) = player_query.get_single_mut() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    for (transform, spawn) in spawn_query.iter() {
        if spawn.spawn_name == request.spawn_name {
            info!("Moving player to spawn {}", spawn.spawn_name);
            player_transform.translation = transform.translation;
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;

            commands.entity(player_entity).remove::<RequestedPlayerSpawn>();
            break;
        }
    }
}