use crate::audio::audio_components::AudioMusic;
use bevy::prelude::*;

pub fn music_added_observer(
    added: Trigger<OnAdd, AudioMusic>,
    mut commands: Commands,
    music: Query<Entity, With<AudioMusic>>,
) {
    for music in music.iter() {
        if music !=added.entity() {
            commands.entity(music).despawn_recursive();
        }
    }
}
