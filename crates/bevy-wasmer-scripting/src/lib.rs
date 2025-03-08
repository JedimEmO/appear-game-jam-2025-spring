use bevy::app::{App, Plugin};
use wasmtime::{Config, Engine};
use crate::scripted_entity::WasmEngine;

pub mod scripted_entity;
pub mod wasm_script_asset;

pub struct WasmtimeScriptPlugin;

impl Plugin for WasmtimeScriptPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WasmEngine(Engine::new(Config::new()
            .wasm_threads(false)
            .wasm_component_model(true)).unwrap()));
    }
}