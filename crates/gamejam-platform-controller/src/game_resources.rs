use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res, ResMut};
use crate::game_entities::file_formats::game_entity_definitions::{GameEntityDefinitionFile, GameEntityDefinitionFileHandle};
use crate::graphics::sprite_collection::{AnimatedSpriteFile, AnimatedSpriteFileHandle};

pub fn load_resources(mut commands: Commands, assets: ResMut<AssetServer>) {
    let animated_sprite_file = assets.load::<AnimatedSpriteFile>("sprites/animated.sprites.toml");
    commands.insert_resource(AnimatedSpriteFileHandle(animated_sprite_file));
    
    let entity_file = assets.load::<GameEntityDefinitionFile>("entities/entities.toml");
    commands.insert_resource(GameEntityDefinitionFileHandle(entity_file));
}