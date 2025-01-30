use crate::graphics::animation_system::{spawn_animated_sprite_for_entity, SpriteAnimation, SpriteSettings};
use crate::player_components::{Attacking, JumpState, Player, PlayerMovementData, Pogoing};
use crate::player_const_rules::POGO_HIT_KICKBACK_ACCELERATION;
use crate::{AttackDirection, PlayerAssets};
use avian2d::prelude::{LinearVelocity, SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_trauma_shake::Shake;
use std::time::Duration;
use crate::graphics::sprite_collection::SpriteCollection;

pub fn player_attack_start_system(
    mut commands: Commands,
    sprite_collection: Res<SpriteCollection>,
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
    >
) {
    let Ok((entity, attacking, mut velocity, mut animation, player_transform, mut movement_data)) =
        player.get_single_mut()
    else {
        return;
    };

    let is_pogo = attacking.direction == AttackDirection::Down;

    animation.play_animation(
        if is_pogo { 5 } else { 4 } * 4,
        4,
        Duration::from_millis(150),
        false,
    );

    {
        if let Some(bundle) = sprite_collection.create_sprite_animation_bundle(
            "player_attack",
            if attacking.direction == AttackDirection::Down { "down" } else { "horizontal" },
            Duration::from_millis(150),
            false,
            true,
            movement_data.horizontal_direction
        ) {
            let swoosh_entity = commands.spawn(bundle).id();
            commands.entity(entity).add_child(swoosh_entity);
        }
    };

    if is_pogo {
        commands.entity(entity).insert(Pogoing);
    }
}

pub fn player_pogo_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player: Query<
        (
            Entity,
            &mut LinearVelocity,
            &Transform,
            &mut JumpState
        ),
        (With<Player>, With<Pogoing>),
    >,
    mut camera_shake: Query<&mut Shake, With<Camera2d>>,
    spatial_query: SpatialQuery,
) {
    let Ok(mut camera_shake) = camera_shake.get_single_mut() else {
        return;
    };

    for (entity, mut velocity, transform, mut jump_state) in player.iter_mut() {

        // Process pogo
        let hits = spatial_query.ray_hits(
            transform.translation.truncate(),
            Dir2::NEG_Y,
            30.,
            1,
            false,
            &SpatialQueryFilter::from_mask(0b01000),
        );

        if !hits.is_empty() {
            jump_state.last_grounded_time = Some(time.elapsed_secs_f64());
            commands.entity(entity).remove::<Pogoing>();

            camera_shake.add_trauma(0.2);
            let kickback = Vec2::Y;
            velocity.y += kickback.y * POGO_HIT_KICKBACK_ACCELERATION;
        }
    }
}