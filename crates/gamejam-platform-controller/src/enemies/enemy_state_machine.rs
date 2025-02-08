use crate::enemies::{Dying, Sleeping};
use crate::enemies::{Enemy, EnemyStateMachine};
use crate::graphics::animation_system::SpriteAnimation;
use crate::graphics::sprite_collection::{AnimatedSprite, SpriteCollection};
use crate::player_components::{Moving, Player};
use avian2d::prelude::{LinearVelocity, SpatialQuery};
use bevy::prelude::*;
use std::time::Duration;

pub fn enemy_state_machine_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteCollection>,
    player: Query<&Transform, With<Player>>,
    mut enemies: Query<
        (
            Entity,
            &mut Enemy,
            &Transform,
            &mut LinearVelocity,
            &mut Sprite,
            &mut SpriteAnimation,
        ),
        (Without<Player>, Without<Sleeping>),
    >,
    spatial_query: SpatialQuery,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };

    for (entity, mut enemy, enemy_transform, mut enemy_lin_vel, mut sprite, mut sprite_animation) in
        enemies.iter_mut()
    {
        let distance_to_player = player_transform
            .translation
            .distance(enemy_transform.translation);


        if sprite_animation.animation_name == "death" && sprite_animation.finished() {
            enemy.state_machine = EnemyStateMachine::Dead;
            enemy_lin_vel.x = 0.;

            (*sprite, *sprite_animation) = assets
                .create_sprite_animation_bundle(
                    "what_sprite",
                    "dead",
                    Duration::from_millis(500),
                    true,
                    false,
                    sprite.flip_x,
                )
                .expect("invalid sprite");
        }

        match enemy.state_machine {
            EnemyStateMachine::Idle => {
                commands.entity(entity).remove::<Moving>();

                if distance_to_player > 200. {
                    continue;
                }

                info!("Attempting to track player");
                enemy.state_machine = EnemyStateMachine::Charging;
            }
            EnemyStateMachine::Charging => {
                commands.entity(entity).insert(Moving);

                let direction = (player_transform.translation.truncate()
                    - enemy_transform.translation.truncate())
                .normalize();

                if distance_to_player >= 200. {
                    enemy.state_machine = EnemyStateMachine::Idle;
                }

                enemy_lin_vel.x = direction.x * 80.;

                sprite.flip_x = direction.x.signum() == 1.;
            }

            EnemyStateMachine::Staggered {
                staggered_at,
                stagger_for,
            } => {
                if time.elapsed_secs() - staggered_at >= stagger_for {
                    enemy.state_machine = EnemyStateMachine::Charging;
                }
            }
            EnemyStateMachine::Dying => {
                if sprite_animation.animation_name != "death" {
                    (*sprite, *sprite_animation) = assets
                        .create_sprite_animation_bundle(
                            "what_sprite",
                            "death",
                            Duration::from_millis(300),
                            false,
                            false,
                            sprite.flip_x,
                        )
                        .expect("invalid sprite");
                }
            }
            EnemyStateMachine::Dead => {}
        }
    }
}

pub fn enemy_dying_observer(
    trigger: Trigger<OnAdd, Dying>,
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Enemy), Added<Dying>>,
) {
    for (entity, mut enemy) in enemies.iter_mut() {
        if entity == trigger.entity() {
            commands.entity(entity).remove::<Dying>();
            enemy.state_machine = EnemyStateMachine::Dying;
        }
    }
}
