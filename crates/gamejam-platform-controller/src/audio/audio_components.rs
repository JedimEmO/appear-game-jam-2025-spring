use bevy::prelude::*;

pub const MAX_AUDIO_LEVEL: f32 = 5.;
pub const MIN_AUDIO_LEVEL: f32 = 0.;

#[derive(Resource)]
pub struct AudioLevels {
    pub global: f32,
    pub music: f32,
    pub effects: f32
}

impl AudioLevels {
    pub fn music_level(&self) -> f32 {
        (self.global + self.music).max(0.).min(MAX_AUDIO_LEVEL)
    }
    
    pub fn effects_level(&self) -> f32 {
        (self.global + self.effects).max(0.).min(MAX_AUDIO_LEVEL)
    }
    
    pub fn increase_global(&mut self) {
        self.global = (self.global + 0.5).min(MAX_AUDIO_LEVEL);
    }

    pub fn decrease_global(&mut self) {
        self.global = (self.global - 0.5).max(MIN_AUDIO_LEVEL);
    }
    
    pub fn increase_music(&mut self) {
        self.music = (self.music + 0.5).min(MAX_AUDIO_LEVEL);
    }
    
    pub fn decrease_music(&mut self) {
        self.music = (self.music - 0.5).max(MIN_AUDIO_LEVEL);
    }
    
    pub fn increase_effects(&mut self) {
        self.effects = (self.effects + 0.5).min(MAX_AUDIO_LEVEL);
    }
    
    pub fn decrease_effects(&mut self) {
        self.effects = (self.effects - 0.5).max(MIN_AUDIO_LEVEL);
    }
}

impl Default for AudioLevels {
    fn default() -> Self {
        Self {
            global: 1.0,
            music: 0.0,
            effects: 0.0,
        }
    }
}

#[derive(Component)]
pub struct AudioMusic;

#[derive(Component)]
pub struct AudioEffect;