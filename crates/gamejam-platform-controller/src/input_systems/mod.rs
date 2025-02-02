use bevy::prelude::Event;
use bevy::math::Vec2;
use crate::AttackDirection;

pub mod gamepad_input;
pub mod keyboard_input_system;

#[derive(Event)]
pub enum PlayerInputAction {
    Horizontal(Vec2),
    Jump,
    JumpAbort,
    Attack(AttackDirection),
    Interact
}