use crate::{AttackDirection, PlayerInputAction};
use bevy::prelude::*;

pub fn gamepad_input_system(
    mut event_sender: EventWriter<PlayerInputAction>,
    gamepad_query: Query<&Gamepad>,
) {
    let mut direction = Vec2::ZERO;
    let Ok(gamepad) = gamepad_query.get_single() else {
        return;
    };

    if gamepad.pressed(GamepadButton::DPadDown) && gamepad.just_pressed(GamepadButton::West) {
        event_sender.send(PlayerInputAction::Attack(AttackDirection::Down));
    } else if gamepad.just_pressed(GamepadButton::West) {
        event_sender.send(PlayerInputAction::Attack(AttackDirection::Sideways));
    }

    if gamepad.pressed(GamepadButton::DPadRight) {
        direction.x = 1.;
    } else if gamepad.pressed(GamepadButton::DPadLeft) {
        direction.x = -1.;
    }

    if direction.length() > 0.1 {
        event_sender.send(PlayerInputAction::Horizontal(direction));
    }

    if gamepad.pressed(GamepadButton::South) {
        event_sender.send(PlayerInputAction::Jump);
    }

    if gamepad.just_released(GamepadButton::South) {
        event_sender.send(PlayerInputAction::JumpAbort);
    }
}
