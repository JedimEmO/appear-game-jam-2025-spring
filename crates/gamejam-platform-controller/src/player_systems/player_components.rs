use crate::player_systems::bonfire::Bonfire;
use crate::timing::timing_component::TimerComponent;
use crate::combat::combat_components::Health;
use crate::combat::combat_components::Stamina;
use crate::combat::attackable::Attackable;
use crate::movement_systems::movement_components::FacingDirection;
use crate::movement_systems::movement_components::MovementData;
use crate::player_const_rules::*;
use crate::AttackDirection;
use avian2d::prelude::*;
use bevy::prelude::*;
use haalka::prelude::{Mutable};
use simple_2d_camera::PixelCameraTracked;

#[derive(Component)]
#[require(
    Transform(|| Transform::from_xyz(32., 0., 6.)),
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::rectangle(9., 30.)),
    CollisionMargin(|| CollisionMargin::from(COLLISION_MARGIN)),
    CollisionLayers(|| CollisionLayers::new(0b00001, 0b00101)),
    ExternalForce(|| ExternalForce::default().with_persistence(false)),
    GravityScale(|| GravityScale::from(FALL_GRAVITY)),
    ShapeCaster(|| ShapeCaster::new(Collider::rectangle(4., 4.), Vec2::ZERO, 0., Dir2::NEG_Y).with_max_distance(40.).with_max_hits(5).with_query_filter(SpatialQueryFilter::from_mask(0b00100))),
    LockedAxes(|| LockedAxes::ROTATION_LOCKED),
    MovementDampeningFactor(|| MovementDampeningFactor(X_DAMPENING_FACTOR)),
    JumpState,
    PixelCameraTracked,
    Friction(|| Friction::new(0.)),
    PlayerActionTracker,
    PlayerMovementData,
    PlayerStats,
    Health(|| Health::default_player()),
    FacingDirection(|| FacingDirection::East),
    MovementData(|| MovementData::default_player()),
    Attackable,
    Stamina(|| Stamina::default_player()),
    TimerComponent,
    Bonfire
)]
pub struct Player;

#[derive(Component, Default)]
pub struct Grounded;

#[derive(Component, Default)]
pub struct AttachedToWall;

#[derive(Component, Default)]
pub struct Moving;

#[derive(Component, Default)]
pub struct PowerupRoll;

#[derive(Component, Default)]
pub struct PowerupPogo;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Pogoing;

#[derive(Component)]
pub struct Attacking {
    pub attack_started_at: f64,
    pub direction: AttackDirection,
}

#[derive(Component, Default)]
pub struct PlayerMovementData {
    pub horizontal_direction: bool,
}

#[derive(Component, Default)]
pub struct AttackSprite;

#[derive(Component, Default)]
pub struct PlayerActionTracker {
    pub last_attack_at: Option<f64>,
}

#[derive(Component, Default)]
pub struct JumpState {
    pub used: u8,
    pub left_ground_at: Option<f64>,
    pub last_grounded_time: Option<f64>,
    pub jump_start_requested_at: Option<f32>,
}

impl JumpState {
    pub fn abort_jump(&mut self) {
        self.left_ground_at = Some(0.);
        self.used = 0;
    }
}

#[derive(Component)]
pub struct MovementDampeningFactor(pub f32);

#[derive(Component)]
pub struct PlayerStats {
    pub max_health: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self { max_health: 60}
    }
}

// UI sync
#[derive(Component, Default, Clone)]
pub struct PlayerStatsMutable {
    pub health: StatBarMutables,
    pub stamina: StatBarMutables,
    pub has_pogo: Mutable<bool>,
    pub has_rolling: Mutable<bool>,
}

#[derive(Default, Clone)]
pub struct StatBarMutables {
    pub current: Mutable<u32>,
    pub max: Mutable<u32>,
    pub newly_consumed: Mutable<u32>
}