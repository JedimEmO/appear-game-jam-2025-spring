pub mod scripted_game_entity;

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
