use std::collections::BTreeMap;
use bevy::asset::{Asset, Handle};
use bevy::prelude::{Resource, TypePath};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct GameEntityDefinitionFile {
    pub entities: BTreeMap<String, GameEntityDefinition>
}

#[derive(Serialize, Deserialize)]
pub struct GameEntityDefinition {
    /// Identity used to reference this definition from the editor entity (also key of the map in the file)
    pub id: String,
    /// Used for searching in the editor
    pub tags: Vec<String>,
    pub idle_animation: Option<AnimationDescription>,
    pub distance_based_animations: Option<BTreeMap<String, AnimationDescription>>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AnimationDescription {
    pub sprite_name: String,
    pub animation_name: String,
    pub duration_millis: u64,
}

#[derive(Resource)]
pub struct GameEntityDefinitionFileHandle(pub Handle<GameEntityDefinitionFile>);