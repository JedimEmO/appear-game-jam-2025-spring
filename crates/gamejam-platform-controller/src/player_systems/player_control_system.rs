use crate::graphics::animation_system::SpriteAnimation;
use crate::input_systems::PlayerInputAction;
use crate::ldtk_entities::interactable::{InteractableInRange, Interacted};
use crate::movement_systems::movement_components::FacingDirection;
use crate::player_const_rules::{
    ACCELERATION, FALL_GRAVITY, JUMP_SPEED, MAX_JUMP_ACCELERATION_TIME, MAX_SPEED, MAX_Y_SPEED,
    PLAYER_ATTACK_DELAY_SECONDS,
};
use crate::player_systems::player_components::{
    Attacking, Grounded, JumpState, Moving, Player, PlayerActionTracker, PlayerMovementData,
    Pogoing,
};
use avian2d::math::AdjustPrecision;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::{LevelIid, Respawn};
use std::time::Duration;
use crate::combat::combat_components::Stamina;

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
            &mut SpriteAnimation,
            &mut Sprite,
            Option<&Attacking>,
            &mut PlayerActionTracker,
            &mut PlayerMovementData,
            &mut FacingDirection,
            &mut Stamina
        ),
        With<Player>,
    >,
    level: Query<(Entity, &LevelIid)>,
    interactables: Query<Entity, With<InteractableInRange>>,
) {
    let delta_t = time.delta_secs_f64().adjust_precision();

    for (
        entity,
        mut linear_velocity,
        grounded,
        mut jump_state,
        mut animation,
        mut sprite,
        attacking,
        mut player_actions,
        mut movement_data,
        mut facing_direction,
        mut stamina
    ) in player_velocity.iter_mut()
    {
        linear_velocity.y = linear_velocity.y.clamp(-MAX_Y_SPEED, MAX_Y_SPEED);

        if let Some(_attacking) = attacking {
            if animation.finished() {
                animation.play_animation(0, 4, Duration::from_millis(1000), true);
                commands.entity(entity).remove::<Attacking>();
                commands.entity(entity).remove::<Pogoing>();
            }

            continue;
        }

        let mut still_moving = false;

        for movement_action in movement_events.read() {
            match movement_action {
                PlayerInputAction::Horizontal(dir) => {
                    still_moving = true;
                    if grounded.is_some() {
                        if animation.animation_start_index != 4 {
                            animation.play_animation(4, 4, Duration::from_millis(500), true);
                        }
                    }

                    let reverse_factor = if linear_velocity.x.signum() != dir.x.signum() {
                        FALL_GRAVITY
                    } else {
                        1.
                    };

                    linear_velocity.x += dir.x * ACCELERATION * delta_t * reverse_factor;
                    linear_velocity.y += dir.y * ACCELERATION * delta_t * reverse_factor;

                    linear_velocity.x = linear_velocity.x.clamp(-MAX_SPEED, MAX_SPEED);

                    *facing_direction = match dir.x {
                        x if x < 0. => FacingDirection::West,
                        _ => FacingDirection::East,
                    };

                    movement_data.horizontal_direction = dir.x < 0.;
                    sprite.flip_x = movement_data.horizontal_direction;
                }
                PlayerInputAction::Jump => {
                    do_jump(&time, &mut linear_velocity, grounded, &mut jump_state);
                }
                PlayerInputAction::JumpStart => {
                    if jump_state.jump_start_requested_at.is_none() {
                        jump_state.jump_start_requested_at = Some(time.elapsed_secs());
                    }

                    do_jump(&time, &mut linear_velocity, grounded, &mut jump_state);
                }
                PlayerInputAction::JumpAbort => {
                    if linear_velocity.y > 0.5 {
                        linear_velocity.y = 0.;
                        jump_state.abort_jump();
                    }
                }
                PlayerInputAction::Attack(direction) => {
                    if stamina.current_stamina < 25 {
                        continue;
                    }
                    
                    stamina.current_stamina -= 25;
                    stamina.newly_consumed_stamina += 25;
                    
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
                PlayerInputAction::Interact => {
                    if let Ok(interactable_entity) = interactables.get_single() {
                        commands.entity(interactable_entity).insert(Interacted);
                    }
                }
                PlayerInputAction::ReloadLevel => {
                    let (level, _) = level.single();
                    commands.entity(level).insert(Respawn);
                }
            }
        }

        if jump_state.left_ground_at.is_some()
            && attacking.is_none()
            && animation.animation_start_index != 12
        {
            animation.play_animation(12, 4, Duration::from_millis(500), true);
        }

        if still_moving {
            commands.entity(entity).insert(Moving);
        } else {
            if attacking.is_none() && jump_state.left_ground_at.is_none() {
                animation.play_animation(0, 4, Duration::from_millis(500), true);
            }

            commands.entity(entity).remove::<Moving>();
        }
    }
}

fn do_jump(
    time: &Res<Time>,
    linear_velocity: &mut Mut<LinearVelocity>,
    grounded: Option<&Grounded>,
    jump_state: &mut Mut<JumpState>,
) {
    let now = time.elapsed_secs_f64();
    let left_ground_at = jump_state.left_ground_at;
    let is_grounded = grounded.is_some();

    let coyote_time_delta = now - jump_state.last_grounded_time.unwrap_or(0.);
    let can_coyote_jump = coyote_time_delta <= 0.1;

    let start = match jump_state.jump_start_requested_at {
        Some(time_requested) => time.elapsed_secs() - time_requested < 0.3,
        _ => false,
    };

    if start && (is_grounded || can_coyote_jump && jump_state.used == 0) {
        jump_state.used = 1;
        jump_state.left_ground_at = Some(now);
        linear_velocity.y = JUMP_SPEED;
        jump_state.used += 1;
        jump_state.jump_start_requested_at = None;
    } else if left_ground_at.is_some() && now - left_ground_at.unwrap() < MAX_JUMP_ACCELERATION_TIME
    {
        linear_velocity.y = JUMP_SPEED;
    }
}
