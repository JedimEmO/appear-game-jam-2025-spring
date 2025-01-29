use crate::graphics::animation_system::{spawn_animated_sprite_for_entity, SpriteAnimation, SpriteSettings};
use crate::player_components::{Attacking, Player, PlayerMovementData};
use crate::player_const_rules::POGO_HIT_KICKBACK_ACCELERATION;
use crate::{AttackDirection, PlayerAssets};
use avian2d::prelude::{LinearVelocity, SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_trauma_shake::Shake;
use std::time::Duration;

pub fn player_attack_start_system(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut player: Query<
        (
            Entity,
            &Attacking,
            &mut LinearVelocity,
            &mut SpriteAnimation,
            &Transform,
            &mut PlayerMovementData,
        ),
        (With<Player>, Added<Attacking>),
    >,
    mut camera_shake: Query<&mut Shake, With<Camera2d>>,
    spatial_query: SpatialQuery,
) {
    let Ok((entity, attacking, mut velocity, mut animation, player_transform, mut movement_data)) =
        player.get_single_mut()
    else {
        return;
    };

    let Ok(mut camera_shake) = camera_shake.get_single_mut() else {
        return;
    };

    let is_pogo = attacking.direction == AttackDirection::Down;

    animation.play_animation(
        if is_pogo { 5 } else { 4 } * 4,
        4,
        Duration::from_millis(150),
        false,
    );

    let child_id = {
        let mut swoosh_entity = commands.spawn(());

        spawn_animated_sprite_for_entity(
            &mut swoosh_entity,
            player_assets.player_attack.clone(),
            player_assets.player_attack_layout.clone(),
            if attacking.direction == AttackDirection::Down { 0 } else { 5 },
            5,
            Duration::from_millis(150),
            SpriteSettings {
                despawn_finished: true,
                flip_x: movement_data.horizontal_direction,
                ..default()
            }
        );

        swoosh_entity.id()
    };

    commands.entity(entity).add_child(child_id);

    if is_pogo {
        // Process pogo
        let hits = spatial_query.ray_hits(
            player_transform.translation.truncate(),
            Dir2::NEG_Y,
            30.,
            1,
            false,
            &SpatialQueryFilter::from_mask(0b01000),
        );

        if !hits.is_empty() {
            camera_shake.add_trauma(0.2);
            let kickback = Vec2::Y;
            velocity.y += kickback.y * POGO_HIT_KICKBACK_ACCELERATION;
        }
    }
}
