pub mod attackable;
pub mod enemy;
pub mod enemy_state_machine;
mod hit_points;
mod sleeping;

use crate::enemies::attackable::{Attackable, AttackablePlugin};
use crate::enemies::enemy::spawn_enemy_observer;
use crate::enemies::enemy_state_machine::{enemy_dying_observer, enemy_state_machine_system};
use crate::enemies::sleeping::sleeping_enemy_system;
use crate::player_components::MovementDampeningFactor;
use crate::player_const_rules::{COLLISION_MARGIN, FALL_GRAVITY, X_DAMPENING_FACTOR};
use crate::GameStates;
use avian2d::prelude::*;
use bevy::prelude::*;
use crate::enemies::hit_points::hit_points_system;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AttackablePlugin)
            .add_systems(
                Update,
                (
                    sleeping_enemy_system,
                    enemy_state_machine_system,
                    hit_points_system,
                )
                    .run_if(in_state(GameStates::GameLoop)),
            )
            .add_observer(spawn_enemy_observer)
            .add_observer(enemy_dying_observer);
    }
}

#[derive(Component)]
pub struct HitPoints {
    pub hp: u32,
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
)]
pub struct Enemy {
    pub state_machine: EnemyStateMachine,
}

#[derive(Default)]
pub enum EnemyStateMachine {
    #[default]
    Idle,
    Charging,
    Staggered {
        staggered_at: f32,
        stagger_for: f32,
    },
    Dying,
    Dead
}
