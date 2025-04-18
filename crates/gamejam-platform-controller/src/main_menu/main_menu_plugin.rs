use crate::main_menu::main_menu_system::{enter_main_menu_system, leave_main_menu_system, main_menu_system, ui_audio_levels_system};
use crate::main_menu::menu_input_system::{
    menu_gamepad_input_system, menu_keyboard_input_system, MenuInput,
};
use crate::GameStates;
use bevy::app::{App, FixedUpdate};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};
use crate::main_menu::main_menu_components::UiAudioLevels;

pub struct MainMenuPlugin {}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuInput>()
            .insert_resource(UiAudioLevels::default())
            // input systems must run in Update, since just_pressed* functions
            // get cleared each frame
            .add_systems(
                Update,
                (
                    menu_gamepad_input_system,
                    menu_keyboard_input_system,
                )
                    .run_if(in_state(GameStates::MainMenu)),
            )
            .add_systems(
                FixedUpdate,
                (
                    ui_audio_levels_system,
                    main_menu_system,
                )
                    .run_if(in_state(GameStates::MainMenu)),
            )
            .add_systems(OnExit(GameStates::MainMenu), leave_main_menu_system)
            .add_systems(OnEnter(GameStates::MainMenu), enter_main_menu_system);
    }
}
