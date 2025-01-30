use crate::graphics::animation_system::SpriteAnimation;
use crate::player_components::{Attacking, Direction, Grounded, JumpState, Moving, Player, PlayerActionTracker, PlayerMovementData, Pogoing};
use crate::player_const_rules::{
    ACCELERATION, FALL_GRAVITY, JUMP_SPEED, MAX_JUMP_ACCELERATION_TIME, MAX_SPEED, MAX_Y_SPEED,
    PLAYER_ATTACK_DELAY_SECONDS,
};
use crate::PlayerInputAction;
use avian2d::math::AdjustPrecision;
use avian2d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

pub fn player_control_system(
    mut commands: Commands,
    time: Res<Time>,
    mut movement_events: EventReader<PlayerInputAction>,
    mut player_velocity: Query<
        (
            Entity,
            &mut LinearVelocity,
            Option<&Grounded>,
            &mut JumpState,
            &mut GravityScale,
            &mut SpriteAnimation,
            &mut Sprite,
            Option<&Attacking>,
            &mut PlayerActionTracker,
            &mut PlayerMovementData,
        ),
        With<Player>,
    >,
) {
    let delta_t = time.delta_secs_f64().adjust_precision();

    for (
        entity,
        mut linear_velocity,
        grounded,
        mut jump_state,
        mut gravity_scale,
        mut animation,
        mut sprite,
        attacking,
        mut player_actions,
        mut movement_data,
    ) in player_velocity.iter_mut()
    {
        if !grounded.is_some() {
            gravity_scale.0 = FALL_GRAVITY;
        } else {
            gravity_scale.0 = 1.0;
        }

        linear_velocity.y = linear_velocity.y.clamp(-MAX_Y_SPEED, MAX_Y_SPEED);

        if let Some(_attacking) = attacking {
            if animation.finished() {
                animation.play_animation(0, 4, Duration::from_millis(1000), true);
                commands.entity(entity).remove::<Attacking>();
                commands.entity(entity).remove::<Pogoing>();
            }

            continue;
        }

        if movement_events.is_empty() {
            commands.entity(entity).remove::<Moving>();
            continue;
        } else {
            commands.entity(entity).insert(Moving);
        }

        for movement_action in movement_events.read() {
            match movement_action {
                PlayerInputAction::Horizontal(dir) => {
                    if grounded.is_some() {
                        animation.play_animation(4, 4, Duration::from_millis(1000), true);
                    }

                    let reverse_factor = if linear_velocity.x.signum() != dir.x.signum() {
                        FALL_GRAVITY
                    } else {
                        1.
                    };

                    linear_velocity.x += dir.x * ACCELERATION * delta_t * reverse_factor;
                    linear_velocity.y += dir.y * ACCELERATION * delta_t * reverse_factor;

                    linear_velocity.x = linear_velocity.x.clamp(-MAX_SPEED, MAX_SPEED);
                    movement_data.horizontal_direction = dir.x < 0.;
                    sprite.flip_x = movement_data.horizontal_direction;
                }
                PlayerInputAction::Jump => {
                    do_jump(
                        &time,
                        &mut linear_velocity,
                        grounded,
                        &mut jump_state,
                        &mut gravity_scale,
                    );
                }
                PlayerInputAction::JumpAbort => {
                    gravity_scale.0 = FALL_GRAVITY;
                    linear_velocity.y = 0.;
                }
                PlayerInputAction::Attack(direction) => {
                    let now = time.elapsed_secs_f64();

                    if now - player_actions.last_attack_at.unwrap_or(0.)
                        < PLAYER_ATTACK_DELAY_SECONDS
                    {
                        return;
                    }

                    player_actions.last_attack_at = Some(now);

                    commands.entity(entity).insert(Attacking {
                        attack_started_at: now,
                        direction: *direction,
                    });
                }
            }
        }
    }
}

fn do_jump(
    time: &Res<Time>,
    linear_velocity: &mut Mut<LinearVelocity>,
    grounded: Option<&Grounded>,
    jump_state: &mut Mut<JumpState>,
    gravity_scale: &mut Mut<GravityScale>,
) {
    let now = time.elapsed_secs_f64();
    let left_ground_at = jump_state.left_ground_at;
    let is_grounded = grounded.is_some();

    let coyote_time_delta = now - jump_state.last_grounded_time.unwrap_or(0.);
    let can_coyote_jump = coyote_time_delta <= 0.2;

    if is_grounded || can_coyote_jump && jump_state.used == 0 {
        jump_state.used = 1;
        jump_state.left_ground_at = Some(now);
        linear_velocity.y = JUMP_SPEED;
        gravity_scale.0 = 1.;
    } else if left_ground_at.is_some() && now - left_ground_at.unwrap() < MAX_JUMP_ACCELERATION_TIME
    {
        linear_velocity.y = JUMP_SPEED;
        gravity_scale.0 = 1.;
    }

    jump_state.used += 1;
}
