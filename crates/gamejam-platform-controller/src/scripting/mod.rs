use crate::scripting::scripted_game_entity::{game_entity_script_event_system, ScriptEvent};
use bevy::app::App;
use bevy::prelude::{Plugin, Update};

pub mod scripted_game_entity;
pub struct ScriptedGameEntityPlugin;

impl Plugin for ScriptedGameEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScriptEvent>()
            .add_systems(Update, game_entity_script_event_system);
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
