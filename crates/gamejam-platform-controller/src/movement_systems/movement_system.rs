use crate::movement_systems::movement_components::{
    EntityInput, FacingDirection, Input, MovementData,
};
use crate::player_const_rules::{ACCELERATION, FALL_GRAVITY};
use crate::player_systems::player_components::{Grounded, JumpState, Moving, Player};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::{Commands, Entity, EventReader, Query, Res, Time};
use std::collections::HashSet;

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
        Option<&Player>,
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
            player,
        )) = entities.get_mut(event.entity).ok()
        else {
            continue;
        };

        if player.is_some() {
            continue;
        }

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

                *facing_direction.as_mut() = if delta.x < 0. {
                    FacingDirection::West
                } else {
                    FacingDirection::East
                };
            }

            Input::Jump => {}
        }
    }

    for entity in entities.iter() {
        if entity.6.is_some() {
            continue;
        }

        let mut entity_cmd = commands.entity(entity.0);

        if moving_enemies.contains(&entity.0) {
            entity_cmd.insert(Moving);
        } else {
            entity_cmd.remove::<Moving>();
        }
    }
}
