use crate::combat::combat_components::Stamina;
use crate::graphics::animation_system::SpriteAnimation;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::input_systems::PlayerInputAction;
use crate::ldtk_entities::interactable::{InteractableInRange, Interacted};
use crate::movement_systems::movement_components::{EntityInput, FacingDirection, Input, Rolling};
use crate::player_const_rules::{
    JUMP_SPEED, MAX_JUMP_ACCELERATION_TIME, MAX_Y_SPEED, PLAYER_ATTACK_DELAY_SECONDS,
    PLAYER_ROLL_DURATION,
};
use crate::player_systems::player_components::{
    Attacking, Grounded, JumpState, Moving, Player, PlayerActionTracker, PlayerMovementData,
    Pogoing, PowerupPogo, PowerupRoll,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::{LevelIid, LevelSelection, Respawn};
use std::time::Duration;
use crate::ldtk_entities::player_spawn::RequestedPlayerSpawn;

pub fn player_control_system(
    mut commands: Commands,
    time: Res<Time>,
    sprites: Res<SpriteCollection>,
    mut level_select: ResMut<LevelSelection>,
    mut player_input_reader: EventReader<PlayerInputAction>,
    mut movement_event_writer: EventWriter<EntityInput>,
    mut player_velocity: Query<
        (
            Entity,
            &mut LinearVelocity,
            Option<&Grounded>,
            &mut JumpState,
            &mut SpriteAnimation,
            Option<&Attacking>,
            &mut PlayerActionTracker,
            &mut PlayerMovementData,
            &mut Stamina,
            Option<&Rolling>,
            &FacingDirection,
            Option<&PowerupRoll>,
        ),
        With<Player>,
    >,
    level: Query<(Entity, &LevelIid)>,
    interactables: Query<Entity, With<InteractableInRange>>,
) {
    for (
        entity,
        mut linear_velocity,
        grounded,
        mut jump_state,
        animation,
        attacking,
        mut player_actions,
        mut movement_data,
        mut stamina,
        rolling,
        facing_direction,
        powerup_roll,
    ) in player_velocity.iter_mut()
    {
        linear_velocity.y = linear_velocity.y.clamp(-MAX_Y_SPEED, MAX_Y_SPEED);

        if let Some(_attacking) = attacking {
            if animation.finished() {
                commands.entity(entity).insert(
                    sprites
                        .create_sprite_animation_bundle(
                            "player",
                            "idle",
                            Duration::from_millis(1000),
                            true,
                            false,
                            false,
                        )
                        .unwrap(),
                );
                commands.entity(entity).remove::<Attacking>();
                commands.entity(entity).remove::<Pogoing>();
            }

            continue;
        }

        if rolling.is_some() {
            continue;
        }

        let mut still_moving = false;

        for input_action in player_input_reader.read() {
            match input_action {
                PlayerInputAction::Horizontal(dir) => {
                    still_moving = true;

                    if grounded.is_some() {
                        if animation.animation_name != "run" {
                            commands.entity(entity).insert(
                                sprites
                                    .create_sprite_animation_bundle(
                                        "player",
                                        "run",
                                        Duration::from_millis(500),
                                        true,
                                        false,
                                        facing_direction.to_bool(),
                                    )
                                    .unwrap(),
                            );
                        }
                    }

                    if attacking.is_some() {
                        continue;
                    }

                    movement_event_writer.send(EntityInput {
                        entity,
                        input: Input::Move(*dir),
                    });

                    movement_data.horizontal_direction = dir.x < 0.;
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
                    let now = time.elapsed_secs_f64();

                    if now - player_actions.last_attack_at.unwrap_or(0.)
                        < PLAYER_ATTACK_DELAY_SECONDS
                    {
                        return;
                    }

                    if !stamina.0.try_consume(25) {
                        continue;
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
                &PlayerInputAction::Roll(direction) => {
                    if powerup_roll.is_none() {
                        continue;
                    }

                    if grounded.is_none() || !stamina.0.try_consume(25) {
                        continue;
                    }

                    commands.entity(entity).insert(
                        sprites
                            .create_sprite_animation_bundle(
                                "player",
                                "roll",
                                Duration::from_millis(PLAYER_ROLL_DURATION),
                                false,
                                false,
                                false,
                            )
                            .unwrap(),
                    );

                    movement_event_writer.send(EntityInput {
                        entity,
                        input: Input::Roll {
                            direction,
                            strength: 2500.,
                            duration: Duration::from_millis(PLAYER_ROLL_DURATION),
                        },
                    });

                    return;
                }
                &PlayerInputAction::GoToBoss => {
                    commands.entity(entity).insert((
                        PowerupRoll,
                        PowerupPogo,
                        RequestedPlayerSpawn {
                            spawn_name: "entry".to_string(),
                        },
                    ));
                    *level_select = LevelSelection::index(2);
                }
            }
        }

        if jump_state.left_ground_at.is_some()
            && attacking.is_none()
            && animation.animation_name != "fall"
        {
            commands.entity(entity).insert(
                sprites
                    .create_sprite_animation_bundle(
                        "player",
                        "fall",
                        Duration::from_millis(500),
                        true,
                        false,
                        facing_direction.to_bool(),
                    )
                    .unwrap(),
            );
        }

        if !still_moving {
            if attacking.is_none() && jump_state.left_ground_at.is_none() {
                if animation.animation_name != "idle" {
                    commands.entity(entity).insert(
                        sprites
                            .create_sprite_animation_bundle(
                                "player",
                                "idle",
                                Duration::from_millis(500),
                                true,
                                false,
                                facing_direction.to_bool(),
                            )
                            .unwrap(),
                    );
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
