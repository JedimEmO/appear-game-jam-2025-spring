use crate::player_systems::player_components::{Grounded, MovementDampeningFactor, Moving};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::{Query, Res, Time};
use crate::movement_systems::movement_components::IgnoreDampening;

pub fn movement_dampening_system(
    time: Res<Time>,
    mut query: Query<(
        &mut LinearVelocity,
        &MovementDampeningFactor,
        Option<&Grounded>,
        Option<&Moving>,
        Option<&IgnoreDampening>
    )>,
) {
    for (mut velocity, dampening, grounded, moving, ignore_dampening) in &mut query {
        if ignore_dampening.is_some() {
            continue;
        }
        
        if grounded.is_some() && moving.is_none() {
            velocity.x = 0.;
        } else if moving.is_none() {
            velocity.x *= 1. - dampening.0 * 0.35 * time.delta_secs();
        }
    }
}
