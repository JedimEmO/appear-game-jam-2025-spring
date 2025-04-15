use crate::audio::audio_components::AudioLevels;
use crate::audio::audio_levels_system::audio_levels_system;
use crate::audio::music_system::music_added_observer;
use bevy::prelude::*;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioLevels::default())
            .add_systems(FixedUpdate, audio_levels_system)
            .add_observer(music_added_observer);
    }
}
