use bevy::prelude::*;
use std::time::Duration;
use avian2d::prelude::RigidBody;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::scripting::scripted_game_entity::{EntityScript, ScriptEvent};

#[derive(Component, Default, Debug, Reflect)]
pub struct SpriteAnimation {
    pub timer: Timer,
    pub animation_start_index: u32,
    pub animation_frame: u32,
    pub animation_frame_count: u32,
    pub repeat: bool,
    pub despawn_finished: bool,
    pub animation_name: String,
    pub sprite_size: UVec2,
}

#[cfg(test)]
#[test]
fn path() {
    use crate::enemies::attackable::Attackable;
    use gamejam_bevy_components::Interactable;
    use crate::ui::interactable_hint::InteractableHintComponent;

    assert_eq!(
        vec![
            Sprite::type_path(),
            SpriteAnimation::type_path(),
            Attackable::type_path(),
            Interactable::type_path(),
            InteractableHintComponent::type_path(),
            RigidBody::type_path(),
        ],
        vec![""]);
}

#[derive(Component)]
pub struct SpriteAnimationCompleted;

impl SpriteAnimation {
    pub fn play_animation(
        &mut self,
        animation_index: u32,
        frame_count: u32,
        duration: Duration,
        repeating: bool,
    ) {
        let tick_duration = duration / frame_count;

        self.animation_start_index = animation_index;
        self.animation_frame = 0;
        self.timer = Timer::new(tick_duration, TimerMode::Repeating);
        self.animation_frame_count = frame_count;
        self.repeat = repeating;
    }

    pub fn finished(&self) -> bool {
        self.animation_frame == self.animation_frame_count
    }
}

pub fn animated_sprite_system(
    mut commands: Commands,
    mut event_writer: EventWriter<ScriptEvent>,
    sprites: Res<SpriteCollection>,
    time: Res<Time>,
    mut sprite: Query<(Entity, &mut Sprite, &mut SpriteAnimation, Option<&mut EntityScript>)>,
) {
    for (entity, mut sprite, mut animation, script) in sprite.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            animation.animation_frame += 1;

            if animation.animation_frame >= animation.animation_frame_count {
                if animation.repeat {
                    animation.animation_frame = 0;

                    if let Some(mut script) = script {
                        script.animation_finished(&mut commands, &animation.animation_name, &sprites, &mut event_writer);
                    }
                } else {
                    if animation.despawn_finished {
                        commands.entity(entity).despawn();
                        return;
                    } else {
                        if let Some(mut script) = script {
                            script.animation_finished(&mut commands, &animation.animation_name, &sprites, &mut event_writer);
                        }

                        commands.entity(entity).insert(SpriteAnimationCompleted);
                    }
                }
            }
        }

        let Some(sprite_atlas) = sprite.texture_atlas.as_mut() else {
            return;
        };

        let frame_index = animation
            .animation_frame
            .min(animation.animation_frame_count - 1);
        sprite_atlas.index = (animation.animation_start_index + frame_index) as usize;
    }
}

#[derive(Default, Clone, Copy)]
pub struct SpriteSettings {
    pub repeating: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub despawn_finished: bool,
}

pub fn spawn_animated_sprite_for_entity(
    commands: &mut EntityCommands,
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    animation_index: u32,
    frame_count: u32,
    duration: Duration,
    settings: SpriteSettings,
) {
    let mut animation = SpriteAnimation::default();

    animation.despawn_finished = settings.despawn_finished;

    animation.play_animation(animation_index, frame_count, duration, settings.repeating);

    let mut sprite = Sprite::from_atlas_image(image, TextureAtlas::from(layout.clone()));

    let sprite_atlas = sprite.texture_atlas.as_mut().unwrap();
    sprite_atlas.index = animation.animation_start_index as usize;
    sprite.flip_x = settings.flip_x;

    commands.insert((sprite, animation));
}
