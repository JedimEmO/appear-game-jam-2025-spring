use crate::combat::attackable::{Attackable, Attacked};
use crate::graphics::animation_system::SpriteAnimation;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::player_const_rules::{PLAYER_ATTACK_DURATION, POGO_HIT_KICKBACK_ACCELERATION};
use crate::player_systems::player_components::{
    Attacking, JumpState, Player, PlayerMovementData, Pogoing,
};
use crate::AttackDirection;
use avian2d::position::Position;
use avian2d::prelude::{Collider, LinearVelocity, SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_trauma_shake::Shake;
use std::time::Duration;

pub fn player_attack_start_system(
    mut commands: Commands,
    time: Res<Time>,
    sprite_collection: Res<SpriteCollection>,
    mut player: Query<
        (
            Entity,
            &Attacking,
            &mut LinearVelocity,
            &mut SpriteAnimation,
            &mut JumpState,
            &Transform,
            &mut PlayerMovementData,
        ),
        (With<Player>, Added<Attacking>),
    >,
    mut attackables: Query<
        (Entity, &Transform, &Collider, Option<&mut LinearVelocity>),
        (With<Attackable>, Without<Player>),
    >,
    mut camera_shake: Query<&mut Shake, With<Camera2d>>,
) {
    let Ok((
        entity,
        attacking,
        mut velocity,
        mut animation,
        mut jump_state,
        player_transform,
        movement_data,
    )) = player.get_single_mut()
    else {
        return;
    };

    let Ok(mut camera_shake) = camera_shake.get_single_mut() else {
        return;
    };

    let attack_direction_unit = if movement_data.horizontal_direction {
        -1.
    } else {
        1.
    };

    let is_pogo = attacking.direction == AttackDirection::Down;

    animation.play_animation(
        if is_pogo { 5 } else { 4 } * 4,
        4,
        Duration::from_millis(PLAYER_ATTACK_DURATION),
        false,
    );

    {
        if let Some(bundle) = sprite_collection.create_sprite_animation_bundle(
            "player_attack",
            if attacking.direction == AttackDirection::Down {
                "down"
            } else {
                "horizontal"
            },
            Duration::from_millis(PLAYER_ATTACK_DURATION),
            false,
            true,
            movement_data.horizontal_direction,
        ) {
            let swoosh_entity = commands.spawn(bundle).id();
            commands.entity(entity).add_child(swoosh_entity);
        }
    };

    let mut did_attack_trauma = false;

    // Process hits
    for (attacked_entity, attacked_transform, attacked_collider, attacked_linear_velocity) in
        attackables.iter_mut()
    {
        let attack_ray_direction = if is_pogo {
            Vec2::new(0., -1.)
        } else {
            Vec2::new(attack_direction_unit, 0.)
        };

        let hit = attacked_collider.intersects_ray(
            Position::from_xy(
                attacked_transform.translation.x,
                attacked_transform.translation.y,
            ),
            0.,
            player_transform.translation.truncate(),
            attack_ray_direction,
            if is_pogo { 64. } else { 32. },
        );

        if !hit {
            continue;
        }

        if !did_attack_trauma {
            camera_shake.add_trauma(0.1);
            did_attack_trauma = true;
        }

        commands.entity(attacked_entity).insert(Attacked {
            damage: 5,
            vector: attack_ray_direction,
            origin: player_transform.translation.truncate(),
            force: 2.
        });
        if is_pogo {
            apply_pogo(
                &mut commands,
                &time,
                &mut camera_shake,
                entity,
                &mut velocity,
                &mut jump_state,
            );
        }
    }

    if is_pogo {
        commands.entity(entity).insert(Pogoing);
    }
}

pub fn player_pogo_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player: Query<
        (Entity, &mut LinearVelocity, &Transform, &mut JumpState),
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
            apply_pogo(
                &mut commands,
                &time,
                &mut camera_shake,
                entity,
                &mut velocity,
                &mut jump_state,
            );
        }
    }
}

fn apply_pogo(
    commands: &mut Commands,
    time: &Res<Time>,
    camera_shake: &mut Mut<Shake>,
    entity: Entity,
    velocity: &mut Mut<LinearVelocity>,
    jump_state: &mut Mut<JumpState>,
) {
    jump_state.last_grounded_time = Some(time.elapsed_secs_f64());
    commands.entity(entity).remove::<Pogoing>();

    camera_shake.add_trauma(0.2);
    let kickback = Vec2::Y;
    velocity.y = kickback.y * POGO_HIT_KICKBACK_ACCELERATION;
}
