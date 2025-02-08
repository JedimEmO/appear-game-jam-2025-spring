use crate::enemies::attackable::Attackable;
use crate::enemies::enemy_state_machine::EnemyStateMachine;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::ldtk_entities::chest::Chest;
use crate::ldtk_entities::interactable::Interactable;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use std::time::Duration;
use crate::player_components::MovementDampeningFactor;
use crate::player_const_rules::X_DAMPENING_FACTOR;

#[derive(Component, Default)]
#[require(
    Attackable,
    RigidBody(|| RigidBody::Dynamic),
    CollisionLayers(|| CollisionLayers::new(0b00001, 0b00100)),
    Friction(|| Friction::new(0.)),
    LockedAxes(|| LockedAxes::ROTATION_LOCKED),
    MovementDampeningFactor(|| MovementDampeningFactor(X_DAMPENING_FACTOR)),
)]
pub struct Enemy {
    pub state_machine: EnemyStateMachine,
}

pub fn spawn_enemy_observer(
    trigger: Trigger<OnAdd, Enemy>,
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    mut query: Query<(Entity, &mut Transform, &EntityInstance), Added<Enemy>>,
) {
    for (entity, mut transform, entity_instance) in query.iter_mut() {
        if entity == trigger.entity() {
            info!("Spawning What");
            transform.translation.z = 1.;

            commands.entity(entity).insert((
                Collider::rectangle(32., 32.),
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
        }
    }
}
