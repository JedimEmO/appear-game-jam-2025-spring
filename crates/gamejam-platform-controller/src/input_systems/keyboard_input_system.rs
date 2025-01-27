use crate::MovementAction;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{EventWriter, KeyCode, Res};

pub fn keyboard_input_system(
    mut event_sender: EventWriter<MovementAction>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    if key_input.pressed(KeyCode::ArrowDown) && key_input.just_pressed(KeyCode::KeyF) {
        event_sender.send(MovementAction::PogoAttack);
    } else if key_input.pressed(KeyCode::KeyF) {
        event_sender.send(MovementAction::Attack);
    }

    if key_input.pressed(KeyCode::KeyD) || key_input.pressed(KeyCode::ArrowRight) {
        direction.x = 1.;
    } else if key_input.pressed(KeyCode::KeyA) || key_input.pressed(KeyCode::ArrowLeft) {
        direction.x = -1.;
    }

    if direction.length() > 0.1 {
        event_sender.send(MovementAction::Horizontal(direction));
    }

    if key_input.pressed(KeyCode::Space) {
        event_sender.send(MovementAction::Jump);
    }

    if key_input.just_released(KeyCode::Space) {
        event_sender.send(MovementAction::JumpAbort);
    }
}
