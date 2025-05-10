use crate::scripting::script_entity_command_queue::scripted_entity_command_queue_system;
use crate::scripting::scripted_game_entity::{game_entity_script_event_system, scripted_entity_uniform_system, setup_game_entity_script, tick_scripted_entity_system, GameData, ScriptEvent};
use crate::GameStates;
use bevy::app::{App, FixedUpdate, Startup};
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, Update};

pub mod create_entity_script;
pub mod script_entity_command_queue;
pub mod scripted_game_entity;

pub struct ScriptedGameEntityPlugin;

impl Plugin for ScriptedGameEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScriptEvent>()
            .add_systems(
                FixedUpdate,
                (
                    scripted_entity_uniform_system,
                    game_entity_script_event_system,
                    tick_scripted_entity_system,
                    scripted_entity_command_queue_system,
                )
                    .run_if(in_state(GameStates::GameLoop))
                    .chain(),
            )
            .add_systems(Startup, setup_game_entity_script)
            .insert_resource(GameData::default());
    }
}

pub mod game_entity {
    use bevy::prelude::Component;
    use std::collections::BTreeMap;
    

    #[derive(Component, Default)]
    pub struct EntityScriptContext {
        pub string_values: BTreeMap<String, String>,
    }
}
