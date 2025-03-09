use crate::game_entities::file_formats::game_entity_definitions::{
    GameEntityDefinitionFile, GameEntityDefinitionFileHandle,
};
use crate::graphics::sprite_collection::{AnimatedSpriteFile, AnimatedSpriteFileHandle};
use crate::GameStates;
use bevy::asset::{AssetServer, Handle, LoadedFolder};
use bevy::prelude::{Commands, NextState, Res, ResMut, Resource};

#[derive(Resource)]
pub struct ScriptLoaderHandle(pub Handle<LoadedFolder>);

pub fn load_resources(mut commands: Commands, assets: ResMut<AssetServer>) {
    let animated_sprite_file = assets.load::<AnimatedSpriteFile>("sprites/animated.sprites.toml");
    commands.insert_resource(AnimatedSpriteFileHandle(animated_sprite_file));

    let entity_file = assets.load::<GameEntityDefinitionFile>("entities/entities.toml");
    commands.insert_resource(GameEntityDefinitionFileHandle(entity_file));

    let handle = assets.load_folder("scripts");
    commands.insert_resource(ScriptLoaderHandle(handle));
}

pub fn load_scripts_system(
    assets: ResMut<AssetServer>,
    loading_handle: Res<ScriptLoaderHandle>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    let state = assets.get_load_state(&loading_handle.0).unwrap();

    if state.is_loaded() {
        next_state.set(GameStates::LoadingSprites);
    }
}
