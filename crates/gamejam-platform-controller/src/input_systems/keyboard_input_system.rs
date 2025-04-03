use crate::input_systems::PlayerInputAction;
use crate::AttackDirection;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{EventWriter, KeyCode, Res};
use crate::movement_systems::movement_components::FacingDirection;

pub fn keyboard_input_system(
    mut event_sender: EventWriter<PlayerInputAction>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    if key_input.all_pressed(vec![
        KeyCode::ArrowLeft,
        KeyCode::ControlLeft
    ]) {
        event_sender.send(PlayerInputAction::Roll(FacingDirection::West));
        return;
    }

    if key_input.all_pressed(vec![
        KeyCode::ArrowRight,
        KeyCode::ControlLeft
    ]) {
        event_sender.send(PlayerInputAction::Roll(FacingDirection::East));
        return;
    }

    if key_input.pressed(KeyCode::ArrowDown) && key_input.just_pressed(KeyCode::KeyF) {
        event_sender.send(PlayerInputAction::Attack(AttackDirection::Down));
    } else if key_input.just_pressed(KeyCode::KeyF) {
        event_sender.send(PlayerInputAction::Attack(AttackDirection::Sideways));
    }

    if key_input.pressed(KeyCode::KeyD) || key_input.pressed(KeyCode::ArrowRight) {
        direction.x = 1.;
    } else if key_input.pressed(KeyCode::KeyA) || key_input.pressed(KeyCode::ArrowLeft) {
        direction.x = -1.;
    }

    if key_input.just_pressed(KeyCode::ArrowUp) {
        event_sender.send(PlayerInputAction::Interact);
    }

    if direction.length() > 0.1 {
        event_sender.send(PlayerInputAction::Horizontal(direction));
    }

    if key_input.just_pressed(KeyCode::Space) {
        event_sender.send(PlayerInputAction::JumpStart);
    } else if key_input.pressed(KeyCode::Space) {
        event_sender.send(PlayerInputAction::Jump);
    }

    if key_input.just_released(KeyCode::Space) {
        event_sender.send(PlayerInputAction::JumpAbort);
    }
}
