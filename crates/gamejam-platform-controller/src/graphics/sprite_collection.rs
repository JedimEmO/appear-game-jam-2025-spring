use crate::graphics::animation_system::SpriteAnimation;
use crate::GameStates;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Clone)]
pub struct AnimatedSprite {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animations: BTreeMap<String, AnimationInfo>,
    pub row_width: u32,
    pub sprite_size: UVec2,
}

#[derive(Resource, Default, Clone)]
pub struct SpriteCollection {
    pub sprites: BTreeMap<String, AnimatedSprite>,
}

impl SpriteCollection {
    pub fn create_sprite_animation_bundle(
        &self,
        sprite_name: &str,
        animation_name: &str,
        duration: Duration,
        repeat: bool,
        despawn_finished: bool,
        flip_x: bool,
    ) -> Option<(Sprite, SpriteAnimation)> {
        let Some(sprite_info) = self.sprites.get(sprite_name) else {
            error!("Sprite not found: {}", sprite_name);
            return None;
        };

        let Some(animation) = sprite_info.animations.get(animation_name) else {
            error!("Animation not found: {}", animation_name);
            return None;
        };

        let mut sprite = Sprite::from_atlas_image(
            sprite_info.image.clone(),
            TextureAtlas::from(sprite_info.layout.clone()),
        );

        sprite.flip_x = flip_x;

        let animation = SpriteAnimation {
            timer: Timer::new(duration / animation.frame_count, TimerMode::Repeating),
            animation_start_index: animation.row * sprite_info.row_width
                + animation.frame_start_index,
            animation_frame: 0,
            animation_frame_count: animation.frame_count,
            repeat,
            despawn_finished,
            animation_name: animation_name.to_string(),
            sprite_size: sprite_info.sprite_size,
        };

        sprite.texture_atlas.as_mut().unwrap().index = animation.animation_start_index as usize;

        Some((sprite, animation))
    }

    pub fn create_ui_node_animation_bundle(
        &self,
        sprite_name: &str,
        animation_name: &str,
        duration: Duration,
        repeat: bool,
        despawn_finished: bool,
        flip_x: bool,
    ) -> Option<(ImageNode, SpriteAnimation)> {
        let Some(sprite_info) = self.sprites.get(sprite_name) else {
            error!("Sprite not found: {}", sprite_name);
            return None;
        };

        let Some(animation) = sprite_info.animations.get(animation_name) else {
            error!("Animation not found: {}", animation_name);
            return None;
        };

        let mut sprite = ImageNode::from_atlas_image(
            sprite_info.image.clone(),
            TextureAtlas::from(sprite_info.layout.clone()),
        );

        sprite.flip_x = flip_x;

        let animation = SpriteAnimation {
            timer: Timer::new(duration / animation.frame_count, TimerMode::Repeating),
            animation_start_index: animation.row * sprite_info.row_width
                + animation.frame_start_index,
            animation_frame: 0,
            animation_frame_count: animation.frame_count,
            repeat,
            despawn_finished,
            animation_name: animation_name.to_string(),
            sprite_size: sprite_info.sprite_size,
        };

        sprite.texture_atlas.as_mut().unwrap().index = animation.animation_start_index as usize;

        Some((sprite, animation))
    }
}

pub fn spawn_sprite_collection_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    animated_sprite_file: Res<AnimatedSpriteFileHandle>,
    mut assets: ResMut<Assets<AnimatedSpriteFile>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if let Some(animation_file) = assets.remove(animated_sprite_file.0.id()) {
        info!("Spawning sprite collection system");
        let mut sprite_collection = SpriteCollection::default();

        for (sprite_name, sprite) in animation_file.sprites {
            let mut max_row = 0u32;
            let mut max_frames = 032;

            for (_, anim) in sprite.animations.iter() {
                max_row = anim.row.max(max_row);
                max_frames = (anim.frame_start_index + anim.frame_count).max(max_frames);
            }

            let layout = TextureAtlasLayout::from_grid(
                UVec2::new(sprite.image_width as u32, sprite.image_height as u32),
                max_frames,
                max_row + 1,
                Some(UVec2::new(2, 2)),
                None,
            );
            let layout = asset_server.add(layout);

            let image: Handle<Image> = asset_server.load(sprite.sprite_sheet_file_name);

            sprite_collection.sprites.insert(
                sprite_name,
                AnimatedSprite {
                    image,
                    layout,
                    animations: sprite.animations,
                    row_width: max_frames,
                    sprite_size: UVec2::new(sprite.image_width as u32, sprite.image_height as u32),
                },
            );
        }

        commands.insert_resource(sprite_collection);
        next_state.set(GameStates::Loading);
    }
}

#[derive(Deserialize, Copy, Clone)]
pub struct AnimationInfo {
    pub row: u32,
    pub frame_start_index: u32,
    pub frame_count: u32,
}

#[derive(Deserialize, Asset, TypePath)]
pub struct AnimatedSpriteFileEntry {
    pub sprite_sheet_file_name: String,
    pub animations: BTreeMap<String, AnimationInfo>,
    pub image_width: usize,
    pub image_height: usize,
}

#[derive(Deserialize, Asset, TypePath)]
pub struct AnimatedSpriteFile {
    pub sprites: BTreeMap<String, AnimatedSpriteFileEntry>,
}

#[derive(Resource)]
pub struct AnimatedSpriteFileHandle(pub Handle<AnimatedSpriteFile>);
