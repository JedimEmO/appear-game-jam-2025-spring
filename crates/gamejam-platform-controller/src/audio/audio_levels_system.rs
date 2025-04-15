use crate::audio::audio_components::{AudioEffect, AudioLevels, AudioMusic};
use bevy::prelude::*;

pub fn audio_levels_system(
    levels: ResMut<AudioLevels>,
    mut music_query: Query<&mut AudioSink, (With<AudioMusic>, Without<AudioEffect>)>,
    mut effect_query: Query<&mut AudioSink, (With<AudioEffect>, Without<AudioMusic>)>,
) {
    for sink in music_query.iter_mut() {
        sink.set_volume(levels.music_level());
    }

    for sink in effect_query.iter_mut() {
        sink.set_volume(levels.effects_level());
    }
}
