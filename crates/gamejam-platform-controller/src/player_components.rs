use avian2d::prelude::*;
use bevy::prelude::*;
use haalka::prelude::{Mutable, MutableVec};
use simple_2d_camera::PixelCameraTracked;
use crate::AttackDirection;
use crate::enemies::HitPoints;
use crate::player_const_rules::*;

#[derive(Component)]
#[require(
    Transform(|| Transform::from_xyz(32., 0., 4.)),
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::rectangle(7., 30.)),
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
    HitPoints(|| HitPoints { hp: 3 })
)]
pub struct Player;

#[derive(Component, Default)]
pub struct Grounded;

#[derive(Component, Default)]
pub struct AttachedToWall;

#[derive(Component, Default)]
pub struct Moving;

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
    pub horizontal_direction: bool
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
    pub jump_start_requested_at: Option<f32>
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
    pub max_health: u32
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            max_health: 6
        }
    }
}

// UI sync
#[derive(Component, Default, Clone)]
pub struct PlayerStatsMutable {
    pub hp: Mutable<u32>,
    pub max_hp: Mutable<u32>,
    pub hearts: MutableVec<Mutable<u32>>
}
