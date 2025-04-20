use crate::combat::combat_components::Health;
use crate::combat::Enemy;
use crate::graphics::sprite_collection::SpriteCollection;
use avian2d::collision::Collider;
use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use std::time::Duration;

pub fn spawn_enemy_observer(
    trigger: Trigger<OnAdd, Enemy>,
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    mut query: Query<(Entity, &mut Transform, &EntityInstance, &Enemy), Added<Enemy>>,
) {
    for (entity, mut transform, _entity_instance, enemy) in query.iter_mut() {
        if entity == trigger.entity() {
            transform.translation.z = 1.;

            commands.entity(entity).insert((
                Health::new(enemy.max_hp),
                assets
                    .create_sprite_animation_bundle(
                        "what_sprite",
                        "idle",
                        Duration::from_millis(500),
                        true,
                        false,
                        false,
                    )
                    .unwrap(),
            ));

            commands.entity(entity).insert_if_new(Collider::circle(15.));
        }
    }
}
