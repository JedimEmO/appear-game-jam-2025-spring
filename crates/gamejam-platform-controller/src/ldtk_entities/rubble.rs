use crate::combat::attackable::Attackable;
use crate::combat::{Dying, HitPoints};
use crate::graphics::animation_system::{SpriteAnimation, SpriteAnimationCompleted};
use crate::graphics::sprite_collection::SpriteCollection;
use avian2d::collision::Collider;
use avian2d::prelude::{CollisionLayers, RigidBody};
use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use std::time::Duration;

#[derive(Component, Default, Reflect)]
pub struct Rubble {
    pub collider: bool,
    pub sprite_name: String,
    pub idle_duration: Duration,
    pub death_duration: Duration,
    pub dead_duration: Duration,
}

pub fn spawn_rubble_system(
    _trigger: Trigger<OnAdd, Rubble>,
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    mut query: Query<(Entity, &mut Transform, &EntityInstance, &Rubble), Added<Rubble>>,
) {
    for (entity, _transform, _entity_instance, rubble) in query.iter_mut() {
        let mut entity = commands.entity(entity);

        let (sprite, animation) = assets
            .create_sprite_animation_bundle(
                &rubble.sprite_name,
                "idle",
                rubble.idle_duration,
                true,
                false,
                false,
            )
            .unwrap();

        let (width, height) = (animation.sprite_size.x, animation.sprite_size.y);

        entity.insert((
            sprite,
            animation,
            RigidBody::Static,
            Collider::rectangle(width as f32, height as f32),
            HitPoints { hp: 1 },
            Attackable,
        ));

        if rubble.collider {
            entity.insert((CollisionLayers::new(0b00100, 0b00101),));
        } else {
            entity.insert((CollisionLayers::new(0b00000, 0b00000),));
        }
    }
}

pub fn rubble_dying_observer(
    trigger: Trigger<OnAdd, Dying>,
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    mut enemies: Query<
        (Entity, &mut Sprite, &mut SpriteAnimation, &Rubble),
        (With<Rubble>, Added<Dying>),
    >,
) {
    for (entity, mut sprite, mut animation, rubble) in enemies.iter_mut() {
        if entity == trigger.entity() {
            let mut cmds = commands.entity(entity);
            cmds.remove::<Dying>();
            cmds.remove::<Collider>();
            cmds.remove::<Attackable>();

            (*sprite, *animation) = assets
                .create_sprite_animation_bundle(
                    &rubble.sprite_name,
                    "death",
                    rubble.death_duration,
                    false,
                    false,
                    false,
                )
                .expect("invalid sprite");
        }
    }
}

pub fn rubble_dead_observer(
    trigger: Trigger<OnAdd, SpriteAnimationCompleted>,
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    mut enemies: Query<
        (Entity, &mut Sprite, &mut SpriteAnimation, &Rubble),
        (With<Rubble>, Added<SpriteAnimationCompleted>),
    >,
) {
    for (entity, mut sprite, mut animation, rubble) in enemies.iter_mut() {
        if entity == trigger.entity() {
            let mut cmds = commands.entity(entity);
            cmds.remove::<SpriteAnimationCompleted>();

            (*sprite, *animation) = assets
                .create_sprite_animation_bundle(
                    &rubble.sprite_name,
                    "dead",
                    rubble.dead_duration,
                    true,
                    false,
                    false,
                )
                .expect("invalid sprite");
        }
    }
}
