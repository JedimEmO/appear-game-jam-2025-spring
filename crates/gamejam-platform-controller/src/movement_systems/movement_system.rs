use crate::combat::combat_components::Invulnerable;
use crate::movement_systems::movement_components::{
    ApplyTimedLinearVelocity, EntityInput, FacingDirection, Input, MovementData,
    Rolling,
};
use crate::player_const_rules::{ACCELERATION, FALL_GRAVITY};
use crate::player_systems::player_components::{Grounded, JumpState, Moving, Player};
use crate::timing::timer_system::{add_timed_component_to_entity, add_timer_to_entity};
use crate::timing::timing_component::{TimerComponent, TimerData};
use avian2d::prelude::{Collider, GravityScale, LinearVelocity};
use bevy::math::vec2;
use bevy::prelude::{
    Commands, Entity, EventReader, OnRemove, Query, Res, Time, Timer, Trigger,
    With,
};
use bevy::time::TimerMode;
use std::collections::HashSet;
use std::ops::DerefMut;

pub fn movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut events: EventReader<EntityInput>,
    mut entities: Query<(
        Entity,
        &mut LinearVelocity,
        Option<&Grounded>,
        &mut JumpState,
        &mut FacingDirection,
        &MovementData,
        &mut TimerComponent,
        &Collider,
        &GravityScale,
    )>,
) {
    let delta_t = time.elapsed_secs();
    let mut moving_enemies: HashSet<Entity> = HashSet::default();

    for event in events.read() {
        let Some((
            entity,
            mut linear_velocity,
            _grounded,
            _jump_state,
            mut facing_direction,
            movement_data,
            mut timer,
            _collider,
            gravity,
        )) = entities.get_mut(event.entity).ok()
        else {
            continue;
        };

        match event.input {
            Input::Move(delta) => {
                moving_enemies.insert(entity);

                let reverse_factor = if linear_velocity.x.signum() != delta.x.signum() {
                    FALL_GRAVITY
                } else {
                    1.
                };

                linear_velocity.x += delta.x * ACCELERATION * delta_t * reverse_factor;
                linear_velocity.y += delta.y * ACCELERATION * delta_t * reverse_factor;

                linear_velocity.x = linear_velocity
                    .x
                    .clamp(-movement_data.max_speed, movement_data.max_speed);

                *facing_direction = if delta.x < 0. {
                    FacingDirection::West
                } else {
                    FacingDirection::East
                };
            }

            Input::Jump => {}
            Input::Roll {
                direction,
                strength,
                duration,
            } => {
                linear_velocity.x = 0.;

                let dx = match direction {
                    FacingDirection::West => -1.,
                    FacingDirection::East => 1.,
                };

                *facing_direction = direction;

                let mut entity_commands = commands.entity(entity);

                add_timed_component_to_entity(
                    &mut entity_commands,
                    timer.deref_mut(),
                    (Rolling, Invulnerable),
                    duration,
                );

                let old_gravity = *gravity;
                
                add_timer_to_entity(
                    timer.deref_mut(),
                    TimerData {
                        timer_name: 1,
                        timer: Timer::new(duration, TimerMode::Once),
                        on_expiration: Some(Box::new(move |commands| {
                            commands.insert(old_gravity);
                        })),
                    }
                );

                entity_commands.insert(GravityScale(20.));

                entity_commands.insert((
                    ApplyTimedLinearVelocity {
                        timer: Timer::new(duration, TimerMode::Once),
                        acceleration_function: Box::new(move |_remaining| {
                            vec2(dx, 0.3).normalize() * strength
                        }),
                    },
                    Collider::rectangle(7., 15.),
                ));
            }
        }
    }

    for entity in entities.iter() {
        let mut entity_cmd = commands.entity(entity.0);

        if moving_enemies.contains(&entity.0) {
            entity_cmd.insert(Moving);
        } else {
            entity_cmd.remove::<Moving>();
        }
    }
}

pub fn rolling_removed_observer(
    trigger: Trigger<OnRemove, Rolling>,
    mut commands: Commands,
    mut query: Query<Entity, With<Player>>,
) {
    let entity = trigger.entity();

    if let Ok(_) = query.get_mut(entity) {
        commands.entity(entity).insert(Collider::rectangle(7., 30.));
    }
}
