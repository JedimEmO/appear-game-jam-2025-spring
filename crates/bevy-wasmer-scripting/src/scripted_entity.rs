use bevy::prelude::*;
use wasmtime::Engine;

#[derive(Resource)]
pub struct WasmEngine(pub Engine);
