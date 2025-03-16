use crate::AttackDirection;
use bevy::math::Vec2;
use bevy::prelude::Event;

pub mod gamepad_input;
pub mod keyboard_input_system;

#[derive(Event)]
pub enum PlayerInputAction {
    Horizontal(Vec2),
    Jump,
    JumpStart,
    JumpAbort,
    Attack(AttackDirection),
    Interact,
    ReloadLevel,
}
