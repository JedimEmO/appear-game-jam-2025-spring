use crate::scripted_entity::WasmEngine;
use bevy::app::{App, Plugin};
use wasmtime::{Config, Engine};

pub mod scripted_entity;
pub mod wasm_script_asset;

pub struct WasmtimeScriptPlugin;

impl Plugin for WasmtimeScriptPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WasmEngine(
            Engine::new(Config::new().wasm_component_model(true)).unwrap(),
        ));
    }
}
