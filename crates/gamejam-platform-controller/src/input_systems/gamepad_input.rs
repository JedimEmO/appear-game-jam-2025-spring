use crate::input_systems::PlayerInputAction;
use crate::AttackDirection;
use bevy::prelude::*;

pub fn gamepad_input_system(
    mut event_sender: EventWriter<PlayerInputAction>,
    gamepad_query: Query<&Gamepad>,
) {
    let mut direction = Vec2::ZERO;
    let Ok(gamepad) = gamepad_query.get_single() else {
        return;
    };

    let mut left_stick = gamepad.left_stick();
    let left_stick_down = left_stick.y < -0.4;
    let left_stick_up = left_stick.y > 0.5;

    if (left_stick_down || gamepad.pressed(GamepadButton::DPadDown)) && gamepad.just_pressed(GamepadButton::West) {
        event_sender.send(PlayerInputAction::Attack(AttackDirection::Down));
    } else if gamepad.just_pressed(GamepadButton::West) {
        event_sender.send(PlayerInputAction::Attack(AttackDirection::Sideways));
    }

    if gamepad.just_pressed(GamepadButton::DPadUp) || left_stick_up {
        event_sender.send(PlayerInputAction::Interact);
    }

    if gamepad.pressed(GamepadButton::DPadRight) {
        direction.x = 2.;
    } else if gamepad.pressed(GamepadButton::DPadLeft) {
        direction.x = -2.;
    }

    if direction.length() > 0.1 {
        event_sender.send(PlayerInputAction::Horizontal(direction));
    }


    left_stick.y = 0.;

    if left_stick.length() > 0.3 {
        if left_stick.length() > 0.9 {
            left_stick.x *= 2.
        };

        event_sender.send(PlayerInputAction::Horizontal(left_stick));
    }

    if gamepad.pressed(GamepadButton::South) {
        event_sender.send(PlayerInputAction::Jump);
    }

    if gamepad.just_pressed(GamepadButton::South) {
        event_sender.send(PlayerInputAction::JumpStart);
    }

    if gamepad.just_released(GamepadButton::South) {
        event_sender.send(PlayerInputAction::JumpAbort);
    }

    // Debug

    if gamepad.pressed(GamepadButton::LeftTrigger2) {
        event_sender.send(PlayerInputAction::ReloadLevel);
    }
}
