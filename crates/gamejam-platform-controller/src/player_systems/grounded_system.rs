use crate::player_components::{Attacking, Grounded, JumpState, Moving, Player};
use avian2d::prelude::{LinearVelocity, ShapeHits, SpatialQuery, SpatialQueryFilter};
use bevy::math::Dir2;
use bevy::prelude::{Camera2d, Commands, Entity, Query, Res, Time, Transform, With};
use bevy_trauma_shake::Shake;
use crate::graphics::animation_system::SpriteAnimation;

pub fn grounded_player_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &ShapeHits,
            &mut JumpState,
            &LinearVelocity,
            &mut SpriteAnimation,
            &Transform,
            Option<&Attacking>,
            Option<&Moving>,
        ),
        With<Player>,
    >,
    mut camera_shake: Query<&mut Shake, With<Camera2d>>,
    spatial_query: SpatialQuery,
) {
    for (entity, hits, mut jump_state_data, velocity, mut animation, player_transform, attacking, moving) in
        &mut query
    {
        let is_grounded = hits.iter().any(|hit| {
            hit.point2.y < 0.
                && hit.distance <= 18.
                && hit.normal1.y >= 0.95
                && hit.normal2.y <= -0.95
        });

        let now = time.elapsed_secs_f64();

        if is_grounded {
            if now - jump_state_data.last_grounded_time.unwrap_or(0.) >= 1.5 {
                if let Ok(mut shake) = camera_shake.get_single_mut() {
                    shake.add_trauma(0.3);
                }
            }

            jump_state_data.last_grounded_time = Some(now);

            if attacking.is_none() && moving.is_none() {
                animation.animation_start_index = 0;
            }

            if velocity.y >= 0. {
                commands.entity(entity).insert(Grounded);
                jump_state_data.used = 0;
                jump_state_data.left_ground_at = None;
            }
        } else {
            // Check for collisions when going up
            if velocity.y < 0. {
                let up_hits = spatial_query.ray_hits(
                    player_transform.translation.truncate(),
                    Dir2::Y,
                    50.,
                    2,
                    true,
                    &SpatialQueryFilter::default(),
                );

                if up_hits.iter().any(|hit| hit.distance < 18.) {
                    jump_state_data.left_ground_at = Some(0.);
                }
            }

            commands.entity(entity).remove::<Grounded>();
            
            if attacking.is_none() {
                animation.animation_start_index = 3 * 4;
            }
        }
    }
}
