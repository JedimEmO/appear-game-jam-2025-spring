pub mod attackable;
pub mod combat_components;
pub mod enemy;
mod hit_points;
pub mod scheduled_attack_system;
pub mod stats_system;

use crate::combat::attackable::{Attackable, AttackablePlugin};
use crate::combat::enemy::spawn_enemy_observer;
use crate::combat::hit_points::hit_points_system;
use crate::combat::scheduled_attack_system::scheduled_attack_system;
use crate::movement_systems::movement_components::FacingDirection;
use crate::movement_systems::movement_components::MovementData;
use crate::player_const_rules::{COLLISION_MARGIN, FALL_GRAVITY, X_DAMPENING_FACTOR};
use crate::player_systems::player_components::JumpState;
use crate::player_systems::player_components::MovementDampeningFactor;
use crate::GameStates;
use avian2d::prelude::*;
use bevy::prelude::*;
use crate::combat::stats_system::stats_system;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AttackablePlugin)
            .add_systems(
                FixedUpdate,
                (scheduled_attack_system, hit_points_system, stats_system)
                    .run_if(in_state(GameStates::GameLoop)),
            )
            .add_observer(spawn_enemy_observer);
    }
}

#[derive(Component)]
pub struct Dying;

#[derive(Component)]
pub struct Sleeping;

#[derive(Component, Default)]
#[require(
    Attackable,
    RigidBody(|| RigidBody::Dynamic),
    CollisionLayers(|| CollisionLayers::new(0b01000, 0b00101)),
    CollisionMargin(|| CollisionMargin::from(COLLISION_MARGIN)),
    Friction(|| Friction::new(0.)),
    LockedAxes(|| LockedAxes::ROTATION_LOCKED),
    MovementDampeningFactor(|| MovementDampeningFactor(X_DAMPENING_FACTOR)),
    GravityScale(|| GravityScale::from(FALL_GRAVITY)),
    ShapeCaster(|| ShapeCaster::new(Collider::rectangle(4., 4.), Vec2::ZERO, 0., Dir2::NEG_Y).with_max_distance(40.).with_max_hits(5).with_query_filter(SpatialQueryFilter::from_mask(0b00100))),
    JumpState,
    FacingDirection,
    MovementData(|| MovementData::default_enemy()),
)]
pub struct Enemy;
