use crate::movement_systems::movement_components::MovementData;
use crate::player_systems::player_components::{Grounded, JumpState, Player};
use avian2d::prelude::{LinearVelocity, ShapeHits, SpatialQuery, SpatialQueryFilter};
use bevy::math::Dir2;
use bevy::prelude::{AssetServer, AudioPlayer, Camera2d, Commands, Entity, PlaybackSettings, Query, Res, Time, Transform, With};
use bevy_trauma_shake::Shake;
use crate::audio::audio_components::AudioEffect;

pub fn grounded_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &ShapeHits,
        &mut JumpState,
        &mut LinearVelocity,
        &Transform,
        &MovementData,
        Option<&Player>,
    )>,
    mut camera_shake: Query<&mut Shake, With<Camera2d>>,
    spatial_query: SpatialQuery,
) {
    for (
        entity,
        hits,
        mut jump_state_data,
        mut velocity,
        player_transform,
        movement_data,
        player,
    ) in &mut query
    {
        let is_grounded = hits.iter().any(|hit| {
            hit.point2.y < 0.
                && hit.distance <= movement_data.feet_height
                && hit.normal1.y >= 0.95
                && hit.normal2.y <= -0.95
        });

        let now = time.elapsed_secs_f64();

        if is_grounded {
            if player.is_some() && now - jump_state_data.last_grounded_time.unwrap_or(0.) >= 1.5 {
                if let Ok(mut shake) = camera_shake.get_single_mut() {
                    shake.add_trauma(0.3);
                    
                    commands.spawn((
                        AudioPlayer::new(asset_server.load("audio/player/hit.wav")),
                        AudioEffect,
                        PlaybackSettings::ONCE,
                        ));
                }
            }

            jump_state_data.last_grounded_time = Some(now);
            jump_state_data.jump_start_requested_at = None;

            if velocity.y <= 0. {
                commands.entity(entity).insert(Grounded);
                jump_state_data.used = 0;
                jump_state_data.left_ground_at = None;
            }
        } else {
            if jump_state_data.left_ground_at.is_none() {
                jump_state_data.left_ground_at = Some(now);
            }

            // Check for collisions when going up
            if velocity.y > 0. {
                let up_hits = spatial_query.ray_hits(
                    player_transform.translation.truncate(),
                    Dir2::Y,
                    20.,
                    2,
                    false,
                    &SpatialQueryFilter::from_mask(0b00100),
                );

                if up_hits.iter().any(|hit| hit.distance < 18.) {
                    jump_state_data.abort_jump();
                    velocity.y = velocity.y.clamp(f32::MIN, 0.);
                }
            }

            commands.entity(entity).remove::<Grounded>();
        }
    }
}
