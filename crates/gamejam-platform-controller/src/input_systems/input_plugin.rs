use crate::input_systems::gamepad_input::{gamepad_input_system, GamepadInputStates};
use crate::input_systems::keyboard_input_system::keyboard_input_system;
use crate::GameStates;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GamepadInputStates::default())
            .add_systems(
                FixedUpdate,
                (gamepad_input_system, keyboard_input_system)
                    .run_if(in_state(GameStates::GameLoop)),
            );
    }
}
