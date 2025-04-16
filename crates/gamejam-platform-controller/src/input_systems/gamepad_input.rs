use crate::input_systems::PlayerInputAction;
use crate::AttackDirection;
use bevy::prelude::*;
use crate::movement_systems::movement_components::FacingDirection;

#[derive(Resource, Default)]
pub struct GamepadInputStates {
    pub left_stick_tapped_up: bool
}

pub fn gamepad_input_system(
    mut event_sender: EventWriter<PlayerInputAction>,
    mut stats: ResMut<GamepadInputStates>,
    gamepad_query: Query<&Gamepad>,
) {
    let mut direction = Vec2::ZERO;
    let Ok(gamepad) = gamepad_query.get_single() else {
        return;
    };

    let mut left_stick = gamepad.left_stick();
    let left_stick_down = left_stick.y < -0.4;
    let left_stick_up = left_stick.y > 0.5;

    let left_stick_up_tapped = left_stick_up && !stats.left_stick_tapped_up;

    if !left_stick_up {
        stats.left_stick_tapped_up = false;
    } else {
        stats.left_stick_tapped_up = true;
    }

    if gamepad.pressed(GamepadButton::RightTrigger2) {
        event_sender.send(PlayerInputAction::Roll(FacingDirection::East));
        return;
    }

    if gamepad.pressed(GamepadButton::LeftTrigger2) {
        event_sender.send(PlayerInputAction::Roll(FacingDirection::West));
        return;
    }

    if (left_stick_down || gamepad.pressed(GamepadButton::DPadDown))
        && gamepad.just_pressed(GamepadButton::West)
    {
        event_sender.send(PlayerInputAction::Attack(AttackDirection::Down));
    } else if gamepad.just_pressed(GamepadButton::West) {
        event_sender.send(PlayerInputAction::Attack(AttackDirection::Sideways));
    }

    if gamepad.just_pressed(GamepadButton::DPadUp) || left_stick_up_tapped {
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
}
