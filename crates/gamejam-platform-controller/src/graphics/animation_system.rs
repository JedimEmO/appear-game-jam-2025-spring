use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Default, Debug)]
pub struct SpriteAnimation {
    pub timer: Timer,
    pub animation_start_index: u32,
    pub animation_frame: u32,
    pub animation_frame_count: u32,
    pub repeat: bool,
    pub despawn_finished: bool
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
    time: Res<Time>,
    mut sprite: Query<(Entity, &mut Sprite, &mut SpriteAnimation)>,
) {
    for (entity, mut sprite, mut animation) in sprite.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            animation.animation_frame += 1;

            if animation.animation_frame >= animation.animation_frame_count {
                if animation.repeat {
                    animation.animation_frame = 0;
                } else {
                    if animation.despawn_finished {
                        commands.entity(entity).despawn();
                    } else {
                        commands.entity(entity).insert(SpriteAnimationCompleted);
                    }
                    
                    return;
                }
            }
        }

        let Some(sprite_atlas) = sprite.texture_atlas.as_mut() else {
            return;
        };

        sprite_atlas.index = (animation.animation_start_index + animation.animation_frame) as usize;
    }
}

#[derive(Default, Clone, Copy)]
pub struct SpriteSettings {
    pub repeating: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub despawn_finished: bool
}

pub fn spawn_animated_sprite_for_entity(
    commands: &mut EntityCommands,
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    animation_index: u32,
    frame_count: u32,
    duration: Duration,
    settings: SpriteSettings
) {
    let mut animation = SpriteAnimation::default();

    animation.despawn_finished = settings.despawn_finished;

    animation.play_animation(animation_index, frame_count, duration, settings.repeating);

    let mut sprite = Sprite::from_atlas_image(image, TextureAtlas::from(layout.clone()));

    sprite.flip_x = settings.flip_x;

    commands.insert((sprite, animation));
}
