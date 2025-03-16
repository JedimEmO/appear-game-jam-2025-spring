use crate::scripting::script_entity_command_queue::scripted_entity_command_queue_system;
use crate::scripting::scripted_game_entity::{game_entity_script_event_system, ScriptEvent};
use crate::GameStates;
use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, Update};

pub mod script_entity_command_queue;
pub mod scripted_game_entity;

pub struct ScriptedGameEntityPlugin;

impl Plugin for ScriptedGameEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScriptEvent>().add_systems(
            Update,
            (
                game_entity_script_event_system,
                scripted_entity_command_queue_system,
            )
                .run_if(in_state(GameStates::GameLoop))
                .chain(),
        );
    }
}

pub mod game_entity {
    use bevy::prelude::Component;
    use std::collections::BTreeMap;
    use wasmtime::component::bindgen;

    #[derive(Component, Default)]
    pub struct EntityScriptContext {
        pub string_values: BTreeMap<String, String>,
    }

    bindgen!({
        path: "./src/scripting/components/",
        world: "gamejam:game/game-entity",
    });
}
