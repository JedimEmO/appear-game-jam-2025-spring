use bevy::input::ButtonInput;
use bevy::prelude::{Event, EventWriter, Gamepad, GamepadButton, KeyCode, Query, Res};

#[derive(Event)]
pub enum MenuInput {
    Up,
    Down,
    Left,
    Right,
    Activate,
    Back
}

pub fn menu_gamepad_input_system(
    mut event_sender: EventWriter<MenuInput>,
    gamepad_query: Query<&Gamepad>,
) {
    let Ok(gamepad) = gamepad_query.get_single() else {
        return;
    };

    if gamepad.just_pressed(GamepadButton::DPadUp) {
        event_sender.send(MenuInput::Up);
    } else if gamepad.just_pressed(GamepadButton::DPadDown) {
        event_sender.send(MenuInput::Down);
    } else if gamepad.just_pressed(GamepadButton::DPadLeft) {
        event_sender.send(MenuInput::Left);
    } else if gamepad.just_pressed(GamepadButton::DPadRight) {
        event_sender.send(MenuInput::Right);
    } else if gamepad.just_pressed(GamepadButton::South) {
        event_sender.send(MenuInput::Activate);
    } else if gamepad.just_pressed(GamepadButton::East) {
        event_sender.send(MenuInput::Back);
    }
}

pub fn menu_keyboard_input_system(
    mut event_sender: EventWriter<MenuInput>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::ArrowUp) {
        event_sender.send(MenuInput::Up);
    } else if key_input.just_pressed(KeyCode::ArrowDown) {
        event_sender.send(MenuInput::Down);
    } else if key_input.just_pressed(KeyCode::ArrowLeft) {
        event_sender.send(MenuInput::Left);
    } else if key_input.just_pressed(KeyCode::ArrowRight) {
        event_sender.send(MenuInput::Right);
    } else if key_input.just_pressed(KeyCode::Space) || key_input.just_pressed(KeyCode::Enter) {
        event_sender.send(MenuInput::Activate);
    } else if key_input.just_pressed(KeyCode::Escape) {
        event_sender.send(MenuInput::Back);
    }
}
