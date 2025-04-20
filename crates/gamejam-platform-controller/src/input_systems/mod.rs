use crate::AttackDirection;
use bevy::math::Vec2;
use bevy::prelude::Event;
use crate::movement_systems::movement_components::FacingDirection;

pub mod gamepad_input;
pub mod keyboard_input_system;
pub mod input_plugin;

#[derive(Event)]
pub enum PlayerInputAction {
    Horizontal(Vec2),
    Jump,
    JumpStart,
    JumpAbort,
    Attack(AttackDirection),
    Interact,
    ReloadLevel,
    Roll(FacingDirection),
    GoToBoss
}
