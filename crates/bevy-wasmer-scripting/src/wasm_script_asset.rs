use std::error::Error;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::*;

/// Represents compiled wasm code for a script module
#[derive(Asset, TypePath)]
pub struct WasmScriptModuleBytes {
    pub wasm_module_bytes: Vec<u8>,
    pub aot_component_bytes: Option<Vec<u8>>
}

#[derive(Default)]
pub struct WasmScriptModuleBytesLoader;

impl AssetLoader for WasmScriptModuleBytesLoader {
    type Asset = WasmScriptModuleBytes;
    type Settings = ();
    type Error = Box<dyn Error + Send + Sync + 'static>;

    async fn load(&self, reader: &mut dyn Reader, _settings: &Self::Settings, _load_context: &mut LoadContext<'_>) -> Result<WasmScriptModuleBytes, Self::Error>{
        let mut wasm_module_bytes = Vec::new();
        reader.read_to_end(&mut wasm_module_bytes).await?;
        
        Ok(WasmScriptModuleBytes {
            wasm_module_bytes,
            aot_component_bytes: None,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wasm"]
    }
}