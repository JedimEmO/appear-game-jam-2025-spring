use crate::player_systems::player_components::{Grounded, MovementDampeningFactor, Moving, Player};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::{Query, Res, Time};

pub fn movement_dampening_system(
    time: Res<Time>,
    mut query: Query<(
        &mut LinearVelocity,
        &MovementDampeningFactor,
        Option<&Grounded>,
        Option<&Moving>,
        Option<&Player>,
    )>,
) {
    for (mut velocity, dampening, grounded, moving, _player) in &mut query {
        if grounded.is_some() && moving.is_none() {
            velocity.x = 0.;
        } else if moving.is_none() {
            velocity.x *= 1. - dampening.0 * 0.35 * time.delta_secs();
        }
    }
}
